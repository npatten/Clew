use crate::core::frontmatter::{self, ParsedFile};
use crate::core::increment::{Increment, Status};
use crate::error::ClewError;
use chrono::{SecondsFormat, Utc};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};

const CLEW_DIR: &str = ".clew";
const INCREMENTS_SUBDIR: &str = "increments";
const ARCHIVE_SUBDIR: &str = "archive";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitAction {
    Created,
    Exists,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitItem {
    pub path: &'static str,
    pub action: InitAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitReport {
    pub items: Vec<InitItem>,
}

/// Scaffold `.clew/` in `root`, creating only missing dirs/files.
///
/// Existing paths are never overwritten. The returned report preserves a stable
/// order so the command layer can emit predictable status lines.
pub fn init(root: &Path) -> Result<InitReport, ClewError> {
    let specs = [
        InitSpec::Dir(".clew"),
        InitSpec::Dir(".clew/increments"),
        InitSpec::Dir(".clew/archive"),
        InitSpec::File(".clew/path.md", ""),
        InitSpec::File(
            ".clew/README.md",
            include_str!("../templates/init_readme.md"),
        ),
    ];

    let mut items = Vec::with_capacity(specs.len() + 1);
    for spec in specs {
        let relative = spec.path();
        let path = root.join(relative);
        let action = match spec {
            InitSpec::Dir(_) => create_dir_if_missing(&path)?,
            InitSpec::File(_, contents) => create_file_if_missing(&path, contents)?,
        };
        items.push(InitItem {
            path: relative,
            action,
        });
    }

    let bootstrap = bootstrap_increment_init_action(root)?;
    if let Some(action) = bootstrap {
        items.push(InitItem {
            path: ".clew/increments/0000-bootstrap-clew.md",
            action,
        });
    }

    Ok(InitReport { items })
}

fn bootstrap_increment_init_action(root: &Path) -> Result<Option<InitAction>, ClewError> {
    let relative = ".clew/increments/0000-bootstrap-clew.md";
    let path = root.join(relative);
    if path.exists() {
        return Ok(Some(InitAction::Exists));
    }

    let increments = root.join(CLEW_DIR).join(INCREMENTS_SUBDIR);
    let mut entries = std::fs::read_dir(&increments).map_err(ClewError::Io)?;
    if entries.next().transpose().map_err(ClewError::Io)?.is_some() {
        return Ok(None);
    }

    let contents = bootstrap_increment_contents()?;
    create_file_if_missing(&path, &contents).map(Some)
}

fn bootstrap_increment_contents() -> Result<String, ClewError> {
    // Truncate to whole-second precision to match the frontmatter format
    // contract (RFC 3339 UTC, no subseconds).
    let now = Utc::now()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .expect("RFC 3339 round-trip");

    let increment = Increment {
        id: 0,
        status: Status::Todo,
        parent: None,
        blocked_reason: None,
        abandoned_reason: None,
        tags: Vec::new(),
        created_at: now,
        updated_at: now,
        extra: BTreeMap::new(),
    };

    frontmatter::serialize(&ParsedFile {
        increment,
        body: include_str!("../templates/bootstrap_increment.md").to_string(),
    })
}

fn create_dir_if_missing(path: &Path) -> Result<InitAction, ClewError> {
    if path.exists() {
        return Ok(InitAction::Exists);
    }
    std::fs::create_dir_all(path).map_err(ClewError::Io)?;
    Ok(InitAction::Created)
}

fn create_file_if_missing(path: &Path, contents: &str) -> Result<InitAction, ClewError> {
    if path.exists() {
        return Ok(InitAction::Exists);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(ClewError::Io)?;
    }
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            return Ok(InitAction::Exists);
        }
        Err(e) => return Err(ClewError::Io(e)),
    };
    file.write_all(contents.as_bytes()).map_err(ClewError::Io)?;
    Ok(InitAction::Created)
}

#[derive(Debug, Clone, Copy)]
enum InitSpec {
    Dir(&'static str),
    File(&'static str, &'static str),
}

impl InitSpec {
    fn path(self) -> &'static str {
        match self {
            InitSpec::Dir(path) | InitSpec::File(path, _) => path,
        }
    }
}

/// Returns true if `path` lives under the archive subdir.
pub fn is_archived(path: &Path) -> bool {
    path.parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        == Some(ARCHIVE_SUBDIR)
}

/// Walk up from `start` looking for a `.clew/` directory.
pub fn find_clew_root(start: &Path) -> Result<PathBuf, ClewError> {
    let mut current: Option<&Path> = Some(start);
    while let Some(dir) = current {
        if dir.join(CLEW_DIR).is_dir() {
            return Ok(dir.to_path_buf());
        }
        current = dir.parent();
    }
    Err(ClewError::ClewRootNotFound)
}

/// Resolve a user-supplied id-or-slug to an increment file path.
///
/// Searches `<root>/.clew/increments/` then `<root>/.clew/archive/`. Filenames
/// are expected to be `NNNN-<slug>.md`. The query matches if any of:
/// - the leading 4-digit ID equals the parsed integer form of the query, or
/// - the slug portion (filename without `NNNN-` prefix and `.md` suffix) equals the query.
pub fn resolve(root: &Path, query: &str) -> Result<PathBuf, ClewError> {
    let normalized_query = query.strip_prefix('#').unwrap_or(query);
    if let Ok(qid) = normalized_query.parse() {
        if let Some(path) = find_matching(root, |id, _slug| id == qid)? {
            return Ok(path);
        }
    }

    if let Some((qid, _slug)) = split_filename(normalized_query) {
        if let Some(path) = find_matching(root, |id, _slug| id == qid)? {
            return Ok(path);
        }
    }

    if let Some(path) = find_matching(root, |_id, slug| slug == normalized_query)? {
        return Ok(path);
    }

    Err(ClewError::NotFound(query.to_string()))
}

fn find_matching<F>(root: &Path, mut matches: F) -> Result<Option<PathBuf>, ClewError>
where
    F: FnMut(u32, &str) -> bool,
{
    for subdir in [INCREMENTS_SUBDIR, ARCHIVE_SUBDIR] {
        let dir = root.join(CLEW_DIR).join(subdir);
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(ClewError::Io(e)),
        };

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let stem = match path.file_stem().and_then(|s| s.to_str()) {
                Some(s) => s,
                None => continue,
            };
            let Some((id, slug)) = split_filename(stem) else {
                continue;
            };
            if matches(id, slug) {
                return Ok(Some(path));
            }
        }
    }

    Ok(None)
}

/// Split `0042-add-oauth-routes` into `(42, "add-oauth-routes")`.
/// Returns None if the stem doesn't match the `NNNN-<slug>` shape.
fn split_filename(stem: &str) -> Option<(u32, &str)> {
    let dash = stem.find('-')?;
    let (id_str, rest) = stem.split_at(dash);
    if id_str.len() != 4 {
        return None;
    }
    let id: u32 = id_str.parse().ok()?;
    Some((id, &rest[1..]))
}

/// Read an increment file's contents verbatim.
pub fn read_file(path: &Path) -> Result<String, ClewError> {
    std::fs::read_to_string(path).map_err(ClewError::Io)
}

/// A parsed `NNNN-slug.md` filename from `increments/` or `archive/`.
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub id: u32,
    pub slug: String,
    pub path: PathBuf,
}

/// Scan `<root>/.clew/increments/` and `<root>/.clew/archive/` for all
/// `NNNN-slug.md` files. Filenames that don't match the canonical shape are
/// skipped silently. Used by `clew new` for ID allocation, slug-collision
/// checks, and parent-existence checks (single scan, three queries).
pub fn scan(root: &Path) -> Result<Vec<FileEntry>, ClewError> {
    let mut entries = Vec::new();
    for subdir in [INCREMENTS_SUBDIR, ARCHIVE_SUBDIR] {
        let dir = root.join(CLEW_DIR).join(subdir);
        let read = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(ClewError::Io(e)),
        };
        for entry in read {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let stem = match path.file_stem().and_then(|s| s.to_str()) {
                Some(s) => s,
                None => continue,
            };
            let Some((id, slug)) = split_filename(stem) else {
                continue;
            };
            entries.push(FileEntry {
                id,
                slug: slug.to_string(),
                path,
            });
        }
    }
    Ok(entries)
}

/// A `FileEntry` paired with its parsed frontmatter+body. Layered on top of
/// `scan` so the cheap path stays cheap (ID allocation, slug-collision checks
/// don't need to read every file).
#[derive(Debug)]
pub struct LoadedEntry {
    pub entry: FileEntry,
    pub parsed: ParsedFile,
}

/// Scan + read + parse every increment under both subdirs.
///
/// Frontmatter parse failures are returned as errors rather than skipped: an
/// unreadable increment should stop discovery instead of silently disappearing
/// from `clew list`.
pub fn scan_with_frontmatter(root: &Path) -> Result<Vec<LoadedEntry>, ClewError> {
    let entries = scan(root)?;
    let mut loaded = Vec::with_capacity(entries.len());
    for entry in entries {
        let contents = match std::fs::read_to_string(&entry.path) {
            Ok(c) => c,
            Err(e) => return Err(ClewError::Io(e)),
        };
        let parsed = frontmatter::parse(&contents)
            .map_err(|e| ClewError::Frontmatter(format!("{}: {e}", entry.path.display())))?;
        loaded.push(LoadedEntry { entry, parsed });
    }
    Ok(loaded)
}

/// Write a new increment file under `<root>/.clew/increments/`. Creates the
/// directory if missing. The caller is responsible for building `filename`
/// (canonical `NNNN-slug.md` shape) and `contents` (frontmatter + body).
pub fn write_new_increment(
    root: &Path,
    filename: &str,
    contents: &str,
) -> Result<PathBuf, ClewError> {
    let dir = root.join(CLEW_DIR).join(INCREMENTS_SUBDIR);
    std::fs::create_dir_all(&dir).map_err(ClewError::Io)?;
    let path = dir.join(filename);
    std::fs::write(&path, contents).map_err(ClewError::Io)?;
    Ok(path)
}

/// Overwrite an existing increment file in place. The shared write seam for
/// every state-transition command — keep `commands/` from touching the
/// filesystem directly.
pub fn write_increment(path: &Path, contents: &str) -> Result<(), ClewError> {
    std::fs::write(path, contents).map_err(ClewError::Io)
}

/// Move an increment from `.clew/increments/` to `.clew/archive/`.
pub fn archive_increment(path: &Path) -> Result<PathBuf, ClewError> {
    move_between_increment_dirs(path, ARCHIVE_SUBDIR)
}

/// Move an increment from `.clew/archive/` to `.clew/increments/`.
pub fn unarchive_increment(path: &Path) -> Result<PathBuf, ClewError> {
    move_between_increment_dirs(path, INCREMENTS_SUBDIR)
}

fn move_between_increment_dirs(
    path: &Path,
    destination_subdir: &str,
) -> Result<PathBuf, ClewError> {
    let filename = path
        .file_name()
        .ok_or_else(|| ClewError::Io(std::io::Error::other("increment path has no filename")))?;
    let destination_dir = path
        .parent()
        .and_then(|p| p.parent())
        .map(|clew_dir| clew_dir.join(destination_subdir))
        .ok_or_else(|| ClewError::Io(std::io::Error::other("increment path has no parent")))?;
    std::fs::create_dir_all(&destination_dir).map_err(ClewError::Io)?;
    let destination = destination_dir.join(filename);
    if destination.exists() {
        return Err(ClewError::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("destination already exists: {}", destination.display()),
        )));
    }
    std::fs::rename(path, &destination).map_err(ClewError::Io)?;
    Ok(destination)
}

pub fn read_path_md(root: &Path) -> Result<String, ClewError> {
    let path = root.join(CLEW_DIR).join("path.md");
    match std::fs::read_to_string(&path) {
        Ok(contents) => Ok(contents),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(ClewError::Io(e)),
    }
}

pub fn write_path_md(root: &Path, contents: &str) -> Result<(), ClewError> {
    let path = root.join(CLEW_DIR).join("path.md");
    std::fs::write(path, contents).map_err(ClewError::Io)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_filename_parses_canonical_form() {
        assert_eq!(
            split_filename("0042-add-oauth-routes"),
            Some((42, "add-oauth-routes"))
        );
        assert_eq!(split_filename("0001-x"), Some((1, "x")));
    }

    #[test]
    fn split_filename_rejects_malformed() {
        assert_eq!(split_filename("42-foo"), None);
        assert_eq!(split_filename("00042-foo"), None);
        assert_eq!(split_filename("noprefix"), None);
        assert_eq!(split_filename("abcd-foo"), None);
    }

    #[test]
    fn resolve_prioritizes_numeric_id_over_numeric_slug() {
        let temp = assert_fs::TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path().join(".clew/increments")).unwrap();
        std::fs::write(
            temp.path().join(".clew/increments/0001-42.md"),
            "slug match",
        )
        .unwrap();
        std::fs::write(
            temp.path().join(".clew/increments/0042-real.md"),
            "id match",
        )
        .unwrap();

        let resolved = resolve(temp.path(), "42").unwrap();

        assert_eq!(
            resolved.file_name().and_then(|s| s.to_str()),
            Some("0042-real.md")
        );
    }

    #[test]
    fn resolve_accepts_canonical_reference_form() {
        let temp = assert_fs::TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path().join(".clew/increments")).unwrap();
        std::fs::write(
            temp.path().join(".clew/increments/0042-real.md"),
            "id match",
        )
        .unwrap();

        let resolved = resolve(temp.path(), "#0042").unwrap();

        assert_eq!(
            resolved.file_name().and_then(|s| s.to_str()),
            Some("0042-real.md")
        );
    }

    #[test]
    fn resolve_accepts_canonical_reference_with_slug() {
        let temp = assert_fs::TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path().join(".clew/increments")).unwrap();
        std::fs::write(
            temp.path().join(".clew/increments/0042-add-oauth.md"),
            "id match",
        )
        .unwrap();

        let resolved = resolve(temp.path(), "#0042-add-oauth").unwrap();

        assert_eq!(
            resolved.file_name().and_then(|s| s.to_str()),
            Some("0042-add-oauth.md")
        );
    }

    #[test]
    fn archive_increment_errors_before_overwriting_existing_destination() {
        let temp = assert_fs::TempDir::new().unwrap();
        let increments = temp.path().join(".clew/increments");
        let archive = temp.path().join(".clew/archive");
        std::fs::create_dir_all(&increments).unwrap();
        std::fs::create_dir_all(&archive).unwrap();
        let source = increments.join("0001-a.md");
        let destination = archive.join("0001-a.md");
        std::fs::write(&source, "source").unwrap();
        std::fs::write(&destination, "destination").unwrap();

        let err = archive_increment(&source).unwrap_err();

        assert!(matches!(err, ClewError::Io(e) if e.kind() == std::io::ErrorKind::AlreadyExists));
        assert_eq!(std::fs::read_to_string(&source).unwrap(), "source");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "destination"
        );
    }

    #[test]
    fn unarchive_increment_errors_before_overwriting_existing_destination() {
        let temp = assert_fs::TempDir::new().unwrap();
        let increments = temp.path().join(".clew/increments");
        let archive = temp.path().join(".clew/archive");
        std::fs::create_dir_all(&increments).unwrap();
        std::fs::create_dir_all(&archive).unwrap();
        let source = archive.join("0001-a.md");
        let destination = increments.join("0001-a.md");
        std::fs::write(&source, "source").unwrap();
        std::fs::write(&destination, "destination").unwrap();

        let err = unarchive_increment(&source).unwrap_err();

        assert!(matches!(err, ClewError::Io(e) if e.kind() == std::io::ErrorKind::AlreadyExists));
        assert_eq!(std::fs::read_to_string(&source).unwrap(), "source");
        assert_eq!(
            std::fs::read_to_string(&destination).unwrap(),
            "destination"
        );
    }
}

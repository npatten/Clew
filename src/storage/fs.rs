use crate::core::frontmatter::{self, ParsedFile};
use crate::error::ClewError;
use std::path::{Path, PathBuf};

const CLEW_DIR: &str = ".clew";
const INCREMENTS_SUBDIR: &str = "increments";
const ARCHIVE_SUBDIR: &str = "archive";

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
    if let Ok(qid) = query.parse() {
        if let Some(path) = find_matching(root, |id, _slug| id == qid)? {
            return Ok(path);
        }
    }

    if let Some(path) = find_matching(root, |_id, slug| slug == query)? {
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
}

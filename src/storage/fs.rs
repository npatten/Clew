use crate::error::ClewError;
use std::path::{Path, PathBuf};

const CLEW_DIR: &str = ".clew";
const INCREMENTS_SUBDIR: &str = "increments";
const ARCHIVE_SUBDIR: &str = "archive";

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
    let id_query: Option<u32> = query.parse().ok();
    let slug_query = query;

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
            let Some((id_part, slug_part)) = split_filename(stem) else {
                continue;
            };
            if let Some(qid) = id_query {
                if id_part == qid {
                    return Ok(path);
                }
            }
            if slug_part == slug_query {
                return Ok(path);
            }
        }
    }

    Err(ClewError::NotFound(query.to_string()))
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
}

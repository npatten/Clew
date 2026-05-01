use crate::core::{increment::Status, path};
use crate::error::ClewError;
use crate::storage::fs;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;

pub fn run(start: bool) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;
    let id = select_next(&root)?;

    let id = if start {
        // `start::start` is idempotent: already-in-progress increments are a
        // no-op self-loop, not an error.
        crate::commands::start::start(&root, &id.to_string())?.id
    } else {
        id
    };

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{id:04}").map_err(ClewError::Io)?;
    Ok(())
}

fn select_next(root: &Path) -> Result<u32, ClewError> {
    let path_md = fs::read_path_md(root)?;
    let path_refs = path::references(&path_md);
    if path_refs.is_empty() {
        return oldest_todo(root);
    }

    let loaded = fs::scan_with_frontmatter(root)?;
    let slugs_by_id = loaded
        .iter()
        .map(|entry| (entry.entry.id, entry.entry.slug.clone()))
        .collect::<BTreeMap<_, _>>();
    let entries_by_id = loaded
        .into_iter()
        .map(|entry| (entry.entry.id, entry))
        .collect::<BTreeMap<_, _>>();

    let mut repaired_path = path_md;
    let mut removed_stale_membership = false;

    for id in path_refs {
        let Some(entry) = entries_by_id.get(&id) else {
            return Err(ClewError::NotFound(id.to_string()));
        };

        let status = entry.parsed.increment.status.clone();
        let archived = fs::is_archived(&entry.entry.path);
        if archived || matches!(status, Status::Done | Status::Abandoned) {
            repaired_path = path::remove(&repaired_path, id);
            removed_stale_membership = true;
            warn_removed_path_entry(id, &entry.entry.slug, archived, status)?;
            continue;
        }

        if removed_stale_membership {
            write_normalized_path(root, &repaired_path, &slugs_by_id)?;
        }
        return Ok(id);
    }

    if removed_stale_membership {
        write_normalized_path(root, &repaired_path, &slugs_by_id)?;
    }
    oldest_todo(root)
}

fn warn_removed_path_entry(
    id: u32,
    slug: &str,
    archived: bool,
    status: Status,
) -> Result<(), ClewError> {
    let reason = if archived {
        "archived".to_string()
    } else {
        format!("terminal ({status})")
    };
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    writeln!(
        handle,
        "warning: removed {reason} path.md entry #{id:04}-{slug}"
    )
    .map_err(ClewError::Io)
}

fn write_normalized_path(
    root: &Path,
    text: &str,
    slugs_by_id: &BTreeMap<u32, String>,
) -> Result<(), ClewError> {
    let normalized = path::normalize(text, slugs_by_id);
    fs::write_path_md(root, &normalized)
}

fn oldest_todo(root: &Path) -> Result<u32, ClewError> {
    fs::scan_with_frontmatter(root)?
        .into_iter()
        .filter(|entry| !fs::is_archived(&entry.entry.path))
        .filter(|entry| entry.parsed.increment.status == Status::Todo)
        .min_by_key(|entry| (entry.parsed.increment.created_at, entry.entry.id))
        .map(|entry| entry.entry.id)
        .ok_or(ClewError::NoNextIncrement)
}

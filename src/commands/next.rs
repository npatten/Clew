use crate::core::{increment::Status, path};
use crate::error::ClewError;
use crate::storage::fs;
use std::io::Write;
use std::path::Path;

pub fn run(start: bool) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;
    let id = select_next(&root)?;

    let id = if start {
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
    if let Some(id) = path::references(&path_md).into_iter().next() {
        ensure_path_pick_is_todo(root, id)?;
        return Ok(id);
    }

    oldest_todo(root)
}

fn ensure_path_pick_is_todo(root: &Path, id: u32) -> Result<(), ClewError> {
    let query = id.to_string();
    let selected = fs::resolve(root, &query)?;
    let contents = fs::read_file(&selected)?;
    let parsed = crate::core::frontmatter::parse(&contents)?;

    if fs::is_archived(&selected) {
        return Err(ClewError::ArchivedIncrement {
            action: "select",
            id,
        });
    }

    if parsed.increment.status != Status::Todo {
        return Err(ClewError::InvalidTransition {
            from: parsed.increment.status.to_string(),
            to: "next".to_string(),
        });
    }

    Ok(())
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

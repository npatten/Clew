use crate::core::{increment::Status, path};
use crate::error::ClewError;
use crate::storage::fs;
use std::collections::BTreeMap;
use std::io::Write;

pub fn run(tag: Option<&str>, status: Option<&str>, all: bool) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;

    let status_filter = status.map(parse_status_filter).transpose()?;

    let mut loaded = fs::scan_with_frontmatter(&root)?;
    loaded.sort_by_key(|e| e.entry.id);
    if !all {
        let path_order = path::references(&fs::read_path_md(&root)?);
        let ranks: BTreeMap<u32, usize> = path_order
            .into_iter()
            .enumerate()
            .map(|(rank, id)| (id, rank))
            .collect();
        loaded.sort_by_key(|e| {
            (
                ranks.get(&e.entry.id).copied().unwrap_or(usize::MAX),
                e.entry.id,
            )
        });
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for item in loaded {
        let archived = fs::is_archived(&item.entry.path);
        if !all && archived {
            continue;
        }
        if !all && status_filter.is_none() && is_terminal_status(&item.parsed.increment.status) {
            continue;
        }
        if let Some(ref s) = status_filter {
            if &item.parsed.increment.status != s {
                continue;
            }
        }
        if let Some(t) = tag {
            if !item.parsed.increment.tags.iter().any(|x| x == t) {
                continue;
            }
        }
        writeln!(
            handle,
            "{:04} {:<11} {}",
            item.entry.id,
            item.parsed.increment.status.to_string(),
            item.entry.slug
        )
        .map_err(ClewError::Io)?;
    }
    Ok(())
}

fn parse_status_filter(value: &str) -> Result<Status, ClewError> {
    match value {
        "backlog" => Ok(Status::Backlog),
        "todo" => Ok(Status::Todo),
        "in_progress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        "abandoned" => Ok(Status::Abandoned),
        other => Err(ClewError::InvalidStatusFilter(other.to_string())),
    }
}

fn is_terminal_status(status: &Status) -> bool {
    matches!(status, Status::Done | Status::Abandoned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_status_filter_accepts_all_known_values() {
        assert_eq!(parse_status_filter("backlog").unwrap(), Status::Backlog);
        assert_eq!(parse_status_filter("todo").unwrap(), Status::Todo);
        assert_eq!(
            parse_status_filter("in_progress").unwrap(),
            Status::InProgress
        );
        assert_eq!(parse_status_filter("done").unwrap(), Status::Done);
        assert_eq!(parse_status_filter("abandoned").unwrap(), Status::Abandoned);
    }

    #[test]
    fn parse_status_filter_rejects_unknown_value() {
        let err = parse_status_filter("flying").unwrap_err();
        assert!(matches!(err, ClewError::InvalidStatusFilter(_)));
    }
}

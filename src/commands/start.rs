use crate::core::frontmatter;
use crate::core::increment::Status;
use crate::error::ClewError;
use crate::storage::fs;
use chrono::{SecondsFormat, Utc};
use std::io::Write;

pub fn run(query: &str) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;

    let path = fs::resolve(&root, query)?;
    let contents = fs::read_file(&path)?;
    let mut parsed = frontmatter::parse(&contents)?;

    let from = parsed.increment.status.clone();
    if !matches!(from, Status::Backlog | Status::Todo) {
        return Err(ClewError::InvalidTransition {
            from: from.to_string(),
            to: Status::InProgress.to_string(),
        });
    }

    parsed.increment.status = Status::InProgress;
    let now = Utc::now()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .expect("RFC 3339 round-trip");
    parsed.increment.updated_at = now;

    let id = parsed.increment.id;
    let blocked = parsed.increment.blocked_reason.clone();

    let serialized = frontmatter::serialize(&parsed)?;
    fs::write_increment(&path, &serialized)?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "Started #{:04}", id).map_err(ClewError::Io)?;
    if let Some(reason) = blocked {
        writeln!(handle, "warning: #{:04} is blocked: {}", id, reason).map_err(ClewError::Io)?;
    }
    Ok(())
}

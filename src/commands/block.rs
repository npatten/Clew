use crate::core::frontmatter;
use crate::core::increment::Status;
use crate::error::ClewError;
use crate::storage::fs;
use chrono::{SecondsFormat, Utc};
use std::io::Write;

pub fn run(query: &str, reason: &str) -> Result<(), ClewError> {
    let reason = reason.trim();
    if reason.is_empty() {
        return Err(ClewError::EmptyReason);
    }

    let root = fs::find_clew_root(&std::env::current_dir()?)?;
    let path = fs::resolve(&root, query)?;
    let contents = fs::read_file(&path)?;
    let mut parsed = frontmatter::parse(&contents)?;

    ensure_blockable(&parsed, fs::is_archived(&path), "block")?;

    parsed.increment.blocked_reason = Some(reason.to_string());
    parsed.increment.updated_at = Utc::now()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .expect("RFC 3339 round-trip");

    let id = parsed.increment.id;
    let serialized = frontmatter::serialize(&parsed)?;
    fs::write_increment(&path, &serialized)?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "Blocked #{id:04}").map_err(ClewError::Io)
}

pub(crate) fn ensure_blockable(
    parsed: &frontmatter::ParsedFile,
    archived: bool,
    action: &'static str,
) -> Result<(), ClewError> {
    if archived {
        return Err(ClewError::ArchivedIncrement {
            action,
            id: parsed.increment.id,
        });
    }

    if matches!(parsed.increment.status, Status::Done | Status::Abandoned) {
        return Err(ClewError::InvalidTransition {
            from: parsed.increment.status.to_string(),
            to: action.to_string(),
        });
    }

    Ok(())
}

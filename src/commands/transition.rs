use crate::core::frontmatter;
use crate::core::increment::Status;
use crate::error::ClewError;
use crate::storage::fs;
use chrono::{SecondsFormat, Utc};
use std::path::PathBuf;

#[derive(Debug)]
pub struct AppliedTransition {
    pub id: u32,
    pub path: PathBuf,
    pub blocked_reason: Option<String>,
    pub self_loop: bool,
}

pub fn apply(
    query: &str,
    allowed_from: &[Status],
    to: Status,
    tolerate_unarchived_self_loop: bool,
) -> Result<AppliedTransition, ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;

    let path = fs::resolve(&root, query)?;
    let contents = fs::read_file(&path)?;
    let mut parsed = frontmatter::parse(&contents)?;

    let from = parsed.increment.status.clone();
    let self_loop = from == to;
    if self_loop && tolerate_unarchived_self_loop && !fs::is_archived(&path) {
        return Ok(AppliedTransition {
            id: parsed.increment.id,
            path,
            blocked_reason: parsed.increment.blocked_reason,
            self_loop: true,
        });
    }

    if !allowed_from.contains(&from) {
        return Err(ClewError::InvalidTransition {
            from: from.to_string(),
            to: to.to_string(),
        });
    }

    parsed.increment.status = to;
    parsed.increment.updated_at = Utc::now()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .expect("RFC 3339 round-trip");

    let id = parsed.increment.id;
    let blocked_reason = parsed.increment.blocked_reason.clone();
    let serialized = frontmatter::serialize(&parsed)?;
    fs::write_increment(&path, &serialized)?;

    Ok(AppliedTransition {
        id,
        path,
        blocked_reason,
        self_loop: false,
    })
}

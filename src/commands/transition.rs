use crate::core::frontmatter;
use crate::core::increment::Status;
use crate::error::ClewError;
use crate::storage::fs;
use chrono::{SecondsFormat, Utc};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct AppliedTransition {
    pub id: u32,
    pub path: PathBuf,
    pub blocked_reason: Option<String>,
    pub self_loop: bool,
    pub already_archived: bool,
}

pub fn apply(
    root: &Path,
    query: &str,
    allowed_from: &[Status],
    to: Status,
    tolerate_self_loop: bool,
) -> Result<AppliedTransition, ClewError> {
    let path = fs::resolve(root, query)?;
    let contents = fs::read_file(&path)?;
    let mut parsed = frontmatter::parse(&contents)?;

    let from = parsed.increment.status.clone();
    let self_loop = from == to;
    let already_archived = fs::is_archived(&path);
    if self_loop && tolerate_self_loop {
        return Ok(AppliedTransition {
            id: parsed.increment.id,
            path,
            blocked_reason: parsed.increment.blocked_reason,
            self_loop: true,
            already_archived,
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
        already_archived,
    })
}

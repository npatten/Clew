use crate::commands::block::ensure_blockable;
use crate::core::frontmatter;
use crate::error::ClewError;
use crate::storage::fs;
use chrono::{SecondsFormat, Utc};
use std::io::Write;

pub fn run(query: &str) -> Result<(), ClewError> {
    let root = fs::find_clew_root(&std::env::current_dir()?)?;
    let path = fs::resolve(&root, query)?;
    let contents = fs::read_file(&path)?;
    let mut parsed = frontmatter::parse(&contents)?;

    ensure_blockable(&parsed, fs::is_archived(&path), "unblock")?;

    let id = parsed.increment.id;
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();

    if parsed.increment.blocked_reason.is_none() {
        writeln!(handle, "warning: #{id:04} is already unblocked").map_err(ClewError::Io)?;
        return crate::commands::print_result_line(&root, id, &path);
    }

    parsed.increment.blocked_reason = None;
    parsed.increment.updated_at = Utc::now()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .expect("RFC 3339 round-trip");

    let serialized = frontmatter::serialize(&parsed)?;
    fs::write_increment(&path, &serialized)?;

    writeln!(handle, "Unblocked #{id:04}").map_err(ClewError::Io)?;
    crate::commands::print_result_line(&root, id, &path)
}

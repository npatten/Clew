use crate::core::increment::Status;
use crate::error::ClewError;
use crate::storage::fs;
use std::io::Write;

pub fn run(query: &str) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;

    let transition = crate::commands::transition::apply(
        &root,
        query,
        &[Status::Backlog, Status::Todo],
        Status::InProgress,
        false,
    )?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "Started #{:04}", transition.id).map_err(ClewError::Io)?;
    if let Some(reason) = transition.blocked_reason {
        writeln!(
            handle,
            "warning: #{:04} is blocked: {}",
            transition.id, reason
        )
        .map_err(ClewError::Io)?;
    }
    Ok(())
}

use crate::commands::transition::AppliedTransition;
use crate::core::increment::Status;
use crate::error::ClewError;
use crate::storage::fs;
use std::io::Write;
use std::path::Path;

pub fn run(query: &str) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;
    let transition = start(&root, query)?;
    crate::commands::print_result_line(&root, transition.id, &transition.path)
}

pub fn start(root: &Path, query: &str) -> Result<AppliedTransition, ClewError> {
    let transition = crate::commands::transition::apply(
        root,
        query,
        &[Status::Backlog, Status::Todo],
        Status::InProgress,
        false,
    )?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "Started #{:04}", transition.id).map_err(ClewError::Io)?;
    if let Some(reason) = &transition.blocked_reason {
        writeln!(
            handle,
            "warning: #{:04} is blocked: {}",
            transition.id, reason
        )
        .map_err(ClewError::Io)?;
    }
    Ok(transition)
}

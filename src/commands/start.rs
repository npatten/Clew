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
    // Idempotent: an already-`in_progress` increment is a self-loop, not an
    // error. Lets `clew next --start` and direct `clew start` calls converge
    // on the same return value when the work is already in flight.
    let transition = crate::commands::transition::apply(
        root,
        query,
        &[Status::Backlog, Status::Todo],
        Status::InProgress,
        true,
    )?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    if transition.self_loop {
        writeln!(handle, "warning: #{:04} already in progress", transition.id)
            .map_err(ClewError::Io)?;
    } else {
        writeln!(handle, "Started #{:04}", transition.id).map_err(ClewError::Io)?;
    }
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

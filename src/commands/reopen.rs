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
        &[Status::Done, Status::Abandoned],
        Status::Todo,
        true,
    )?;

    if transition.already_archived {
        fs::unarchive_increment(&transition.path)?;
    }

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    if transition.self_loop && transition.already_archived {
        writeln!(
            handle,
            "warning: #{:04} already marked todo; completing unarchive",
            transition.id
        )
        .map_err(ClewError::Io)?;
    } else if transition.self_loop {
        writeln!(handle, "warning: #{:04} already reopened", transition.id)
            .map_err(ClewError::Io)?;
    }
    writeln!(handle, "Reopened #{:04}", transition.id).map_err(ClewError::Io)?;
    Ok(())
}

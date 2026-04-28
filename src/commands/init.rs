use crate::error::ClewError;
use crate::storage::fs::{self, InitAction};
use std::io::Write;

pub fn run() -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let report = fs::init(&cwd)?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    for item in report.items {
        let action = match item.action {
            InitAction::Created => "created",
            InitAction::Exists => "exists",
        };
        writeln!(handle, "{action}: {}", item.path).map_err(ClewError::Io)?;
    }

    Ok(())
}

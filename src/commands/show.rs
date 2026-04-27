use crate::error::ClewError;
use crate::storage::fs;
use std::io::Write;

pub fn run(query: &str) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;
    let path = fs::resolve(&root, query)?;
    let contents = fs::read_file(&path)?;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle
        .write_all(contents.as_bytes())
        .map_err(ClewError::Io)?;
    Ok(())
}

use crate::error::ClewError;
use std::io::Write;
use std::path::{Path, PathBuf};

pub mod abandon;
pub mod block;
pub mod done;
pub mod init;
pub mod lint;
pub mod list;
pub mod new;
pub mod next;
pub mod reopen;
pub mod show;
pub mod start;
pub mod tag;
pub mod transition;
pub mod unblock;
pub mod untag;

pub(crate) fn print_result_line(root: &Path, id: u32, path: &Path) -> Result<(), ClewError> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result_line(root, id, path)).map_err(ClewError::Io)
}

pub(crate) fn result_line(root: &Path, id: u32, path: &Path) -> String {
    format!("#{id:04} {}", repo_relative_path(root, path).display())
}

pub(crate) fn repo_relative_path(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root)
        .expect("increment path was derived from clew root")
        .to_path_buf()
}

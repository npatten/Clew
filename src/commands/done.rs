use crate::core::{increment::Status, path};
use crate::error::ClewError;
use crate::storage::fs;
use std::collections::BTreeMap;
use std::io::Write;

pub fn run(query: &str) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;

    let transition =
        crate::commands::transition::apply(query, &[Status::InProgress], Status::Done, true)?;

    let path_md = fs::read_path_md(&root)?;
    let path_md = path::remove(&path_md, transition.id);
    let slugs_by_id = fs::scan(&root)?
        .into_iter()
        .map(|entry| (entry.id, entry.slug))
        .collect::<BTreeMap<_, _>>();
    let path_md = path::normalize(&path_md, &slugs_by_id);
    fs::write_path_md(&root, &path_md)?;

    fs::archive_increment(&transition.path)?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    if transition.self_loop {
        writeln!(
            handle,
            "warning: #{:04} already marked done; completing archive",
            transition.id
        )
        .map_err(ClewError::Io)?;
    }
    writeln!(handle, "Done #{:04}", transition.id).map_err(ClewError::Io)?;
    Ok(())
}

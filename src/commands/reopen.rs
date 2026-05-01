use crate::core::{increment::Status, path};
use crate::error::ClewError;
use crate::storage::fs;
use std::collections::BTreeMap;
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

    let result_path = if transition.already_archived {
        fs::unarchive_increment(&transition.path)?
    } else {
        transition.path.clone()
    };

    let slugs_by_id = fs::scan(&root)?
        .into_iter()
        .map(|entry| (entry.id, entry.slug))
        .collect::<BTreeMap<_, _>>();
    if let Some(slug) = slugs_by_id.get(&transition.id) {
        let path_md = fs::read_path_md(&root)?;
        // Idempotent on self-loop / already-ranked entries: append only when
        // the reopened ID isn't already ranked, then normalize cosmetic drift.
        let already_ranked = path::references(&path_md).contains(&transition.id);
        let path_md = if already_ranked {
            path_md
        } else {
            path::append_if_ranked(&path_md, transition.id, slug)
        };
        let path_md = path::normalize(&path_md, &slugs_by_id);
        fs::write_path_md(&root, &path_md)?;
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
    crate::commands::print_result_line(&root, transition.id, &result_path)?;
    Ok(())
}

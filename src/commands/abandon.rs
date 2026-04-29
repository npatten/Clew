use crate::core::{frontmatter, increment::Status, path};
use crate::error::ClewError;
use crate::storage::fs;
use std::collections::BTreeMap;
use std::io::Write;

pub fn run(query: &str, reason: Option<&str>) -> Result<(), ClewError> {
    let reason = reason.map(str::trim).filter(|reason| !reason.is_empty());

    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;
    let pre_transition_missing_reason = abandoned_without_reason(&root, query)?;

    let transition = crate::commands::transition::apply_with(
        &root,
        query,
        &[
            Status::Backlog,
            Status::Todo,
            Status::InProgress,
            Status::Done,
        ],
        Status::Abandoned,
        true,
        |parsed| {
            parsed.increment.abandoned_reason = reason.map(ToOwned::to_owned);
        },
    )?;

    let path_md = fs::read_path_md(&root)?;
    let path_md = path::remove(&path_md, transition.id);
    let slugs_by_id = fs::scan(&root)?
        .into_iter()
        .map(|entry| (entry.id, entry.slug))
        .collect::<BTreeMap<_, _>>();
    let path_md = path::normalize(&path_md, &slugs_by_id);

    let result_path = if transition.already_archived {
        transition.path.clone()
    } else {
        fs::archive_increment(&transition.path)?
    };
    fs::write_path_md(&root, &path_md)?;

    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    if transition.already_archived {
        writeln!(handle, "warning: #{:04} already archived", transition.id)
            .map_err(ClewError::Io)?;
    } else if transition.self_loop {
        writeln!(
            handle,
            "warning: #{:04} already marked abandoned; completing archive",
            transition.id
        )
        .map_err(ClewError::Io)?;
    }
    if (reason.is_none() && !transition.self_loop)
        || (transition.self_loop && pre_transition_missing_reason)
    {
        writeln!(
            handle,
            "warning: #{:04} is abandoned without an abandoned_reason",
            transition.id
        )
        .map_err(ClewError::Io)?;
    }
    writeln!(handle, "Abandoned #{:04}", transition.id).map_err(ClewError::Io)?;
    crate::commands::print_result_line(&root, transition.id, &result_path)?;
    Ok(())
}

fn abandoned_without_reason(root: &std::path::Path, query: &str) -> Result<bool, ClewError> {
    let path = fs::resolve(root, query)?;
    let contents = fs::read_file(&path)?;
    let parsed = frontmatter::parse(&contents)?;
    Ok(parsed.increment.status == Status::Abandoned && parsed.increment.abandoned_reason.is_none())
}

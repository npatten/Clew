use crate::core::{increment::Status, path};
use crate::error::ClewError;
use crate::storage::fs;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub fn run() -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;
    let issues = collect_issues(&root)?;

    if issues.is_empty() {
        eprintln!("No lint issues found");
        return Ok(());
    }

    for issue in &issues {
        eprintln!("warning: {issue}");
    }

    Err(ClewError::LintFailed(issues.len()))
}

#[derive(Debug)]
struct LintEntry {
    slug: String,
    status: Status,
    archived: bool,
}

fn collect_issues(root: &Path) -> Result<Vec<String>, ClewError> {
    let loaded = fs::scan_with_frontmatter(root)?;
    let mut issues = Vec::new();

    let mut entries_by_file_id = BTreeMap::new();
    let mut todo_ids = BTreeSet::new();

    for loaded_entry in loaded {
        let file_id = loaded_entry.entry.id;
        let frontmatter_id = loaded_entry.parsed.increment.id;
        let status = &loaded_entry.parsed.increment.status;
        let archived = fs::is_archived(&loaded_entry.entry.path);
        let slug = loaded_entry.entry.slug;

        if file_id != frontmatter_id {
            issues.push(format!(
                "filename #{file_id:04}-{slug} has frontmatter id {frontmatter_id}; make them match before running transition commands"
            ));
        }

        if archived {
            if !matches!(status, Status::Done | Status::Abandoned) {
                issues.push(format!(
                    "#{file_id:04}-{slug} is archived with status {status}; move it back to .clew/increments/ or change status to done/abandoned"
                ));
            }
        } else {
            match status {
                Status::Done => issues.push(format!(
                    "#{file_id:04}-{slug} has status done but is not archived; run `clew done {file_id:04}`"
                )),
                Status::Abandoned => issues.push(format!(
                    "#{file_id:04}-{slug} has status abandoned but is not archived; run `clew abandon {file_id:04}`"
                )),
                Status::Todo => {
                    todo_ids.insert(file_id);
                }
                Status::Backlog | Status::InProgress => {}
            }
        }

        entries_by_file_id.insert(
            file_id,
            LintEntry {
                slug,
                status: status.clone(),
                archived,
            },
        );
    }

    let path_md = fs::read_path_md(root)?;
    let path_refs = path::references(&path_md);
    let path_ref_set: BTreeSet<u32> = path_refs.iter().copied().collect();

    for id in path_refs {
        match entries_by_file_id.get(&id) {
            None => issues.push(format!("path.md references missing #{id:04}")),
            Some(entry) if entry.archived => issues.push(format!(
                "path.md references archived #{id:04}-{}; remove it or run `clew reopen {id:04}`",
                entry.slug
            )),
            Some(entry) if entry.status == Status::Todo => {
                let expected = format!("#{id:04}-{}", entry.slug);
                for line in path_md.lines().filter(|line| line_references(line, id)) {
                    if !line.contains(&expected) {
                        issues.push(format!(
                            "path.md reference #{id:04} is not canonical; expected {expected}"
                        ));
                    }
                }
            }
            Some(entry) => issues.push(format!(
                "path.md references #{id:04}-{} with status {}; expected todo",
                entry.slug, entry.status
            )),
        }
    }

    if !path_ref_set.is_empty() {
        for id in todo_ids.difference(&path_ref_set) {
            if let Some(entry) = entries_by_file_id.get(id) {
                issues.push(format!(
                    "#{id:04}-{} is todo but missing from path.md priority order",
                    entry.slug
                ));
            }
        }
    }

    Ok(issues)
}

fn line_references(line: &str, id: u32) -> bool {
    line.contains(&format!("#{id:04}"))
}

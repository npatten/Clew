use crate::core::frontmatter::{self, ParsedFile};
use crate::core::increment::{Increment, Status};
use crate::core::slug;
use crate::error::ClewError;
use crate::storage::fs;
use chrono::{SecondsFormat, Utc};
use std::collections::BTreeMap;
use std::io::{IsTerminal, Write};

pub fn run(title: &str, ready: bool, parent: Option<u32>) -> Result<(), ClewError> {
    let cwd = std::env::current_dir().map_err(ClewError::Io)?;
    let root = fs::find_clew_root(&cwd)?;

    // Single scan covers ID allocation, slug-collision check, and parent
    // existence — all three need the same `(id, slug)` view of both subdirs.
    let entries = fs::scan(&root)?;

    if let Some(parent_id) = parent {
        if !entries.iter().any(|e| e.id == parent_id) {
            return Err(ClewError::NotFound(format!("parent #{parent_id:04}")));
        }
    }

    let new_slug = slug::generate(title);
    if let Some(existing) = entries.iter().find(|e| e.slug == new_slug) {
        let filename = existing
            .path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        return Err(ClewError::SlugCollision {
            slug: new_slug,
            existing: filename,
        });
    }

    let body = read_stdin_body()?;

    let next_id = entries.iter().map(|e| e.id).max().map_or(1, |m| m + 1);
    // Truncate to whole-second precision to match the frontmatter format
    // contract (RFC 3339 UTC, no subseconds).
    let now = Utc::now().with_timezone(&Utc);
    let now = now
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .expect("RFC 3339 round-trip");

    let increment = Increment {
        id: next_id,
        status: if ready { Status::Todo } else { Status::Backlog },
        parent,
        blocked_reason: None,
        abandoned_reason: None,
        tags: Vec::new(),
        created_at: now,
        updated_at: now,
        extra: BTreeMap::new(),
    };

    let contents = frontmatter::serialize(&ParsedFile { increment, body })?;

    let filename = format!("{:04}-{}.md", next_id, new_slug);
    fs::write_new_increment(&root, &filename, &contents)?;

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{:04}", next_id).map_err(ClewError::Io)?;
    Ok(())
}

fn read_stdin_body() -> Result<String, ClewError> {
    let stdin = std::io::stdin();
    if stdin.is_terminal() {
        return Ok(String::new());
    }

    let body = std::io::read_to_string(stdin).map_err(ClewError::Io)?;
    if body.starts_with("---\n") || body.starts_with("---\r\n") {
        return Err(ClewError::InvalidStdin);
    }

    Ok(body)
}

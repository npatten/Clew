use crate::core::{frontmatter, tag};
use crate::error::ClewError;
use crate::storage::fs;
use chrono::{SecondsFormat, Utc};

pub fn run(query: &str, tags: &[String]) -> Result<(), ClewError> {
    tag::validate_all(tags)?;

    let root = fs::find_clew_root(&std::env::current_dir()?)?;
    let path = fs::resolve(&root, query)?;
    let contents = fs::read_file(&path)?;
    let mut parsed = frontmatter::parse(&contents)?;

    let mut merged = parsed.increment.tags.clone();
    merged.extend(tags.iter().cloned());
    let merged = tag::dedupe_preserving_order(merged);

    if parsed.increment.tags != merged {
        parsed.increment.tags = merged;
        parsed.increment.updated_at = Utc::now()
            .to_rfc3339_opts(SecondsFormat::Secs, true)
            .parse()
            .expect("RFC 3339 round-trip");

        let serialized = frontmatter::serialize(&parsed)?;
        fs::write_increment(&path, &serialized)?;
    }

    crate::commands::print_result_line(&root, parsed.increment.id, &path)
}

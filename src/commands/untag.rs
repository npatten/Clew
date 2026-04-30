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

    let removals = tag::dedupe_preserving_order(tags.to_vec());
    if let Some(missing) = removals.iter().find(|tag| {
        !parsed
            .increment
            .tags
            .iter()
            .any(|existing| existing == *tag)
    }) {
        return Err(ClewError::MissingTag {
            id: parsed.increment.id,
            tag: missing.clone(),
        });
    }

    parsed
        .increment
        .tags
        .retain(|existing| !removals.iter().any(|tag| tag == existing));
    parsed.increment.updated_at = Utc::now()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .parse()
        .expect("RFC 3339 round-trip");

    let serialized = frontmatter::serialize(&parsed)?;
    fs::write_increment(&path, &serialized)?;

    crate::commands::print_result_line(&root, parsed.increment.id, &path)
}

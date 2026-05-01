//! `path.md` is a flat priority list of open increments. One line per entry.
//!
//! Canonical line format:
//!
//! ```text
//! NNNN slug-text [optional free-form trailing prose]
//! ```
//!
//! - `NNNN` is the four-digit increment ID (the authoritative token).
//! - `slug-text` is the increment's current slug; cosmetic, kept in sync by CLI
//!   writes and `clew lint`.
//! - Anything after the slug column is preserved verbatim (user annotations).
//!
//! Status is *not* persisted here — frontmatter remains the source of truth,
//! and `clew list` joins status at render time. See `clew-spec.md` (`path.md`).
//!
//! Lines that don't start with a four-digit token are treated as prose and
//! ignored by the parser.

use std::collections::BTreeMap;

/// Extract ranked ID references from a `path.md` document in document order.
pub fn references(text: &str) -> Vec<u32> {
    text.lines()
        .filter_map(|line| parse_entry(line).map(|e| e.id))
        .collect()
}

/// Parsed canonical view of a single `path.md` entry.
///
/// The second token is always treated as the slug. Pasted `clew list` output
/// (`NNNN status slug`) is handled contextually by `has_status_column` and
/// `normalize`, where the caller knows the increment's current slug.
#[derive(Debug, PartialEq, Eq)]
pub struct Entry<'a> {
    pub id: u32,
    pub slug: Option<&'a str>,
}

/// Parse one line as a canonical entry. Returns `None` for prose / blank lines.
pub fn parse_entry(line: &str) -> Option<Entry<'_>> {
    let tokens = tokens(line);
    let id_token = tokens.first()?;
    if id_token.text.len() != 4 || !id_token.text.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    Some(Entry {
        id: id_token.text.parse().ok()?,
        slug: tokens.get(1).map(|token| token.text),
    })
}

/// True when `line` looks like pasted `clew list` output for this increment:
/// `NNNN status expected_slug`.
///
/// This is intentionally contextual:
///   - token 1 must be a known status word,
///   - token 2 must equal the expected slug, and
///   - token 1 must NOT also equal the expected slug. When the slug itself is
///     a status word (e.g. slug `todo`), a line like `0001 todo p0` or
///     `0001 todo todo` is the canonical 2-column form with an annotation, not
///     pasted list output — we prefer the canonical interpretation so trailing
///     annotations are preserved.
pub fn has_status_column(line: &str, expected_slug: &str) -> bool {
    let tokens = tokens(line);
    let Some(first) = tokens.get(1) else {
        return false;
    };
    if first.text == expected_slug {
        return false;
    }
    is_status_word(first.text)
        && matches!(tokens.get(2), Some(token) if token.text == expected_slug)
}

fn is_status_word(token: &str) -> bool {
    matches!(
        token,
        "backlog" | "todo" | "in_progress" | "done" | "abandoned"
    )
}

#[derive(Debug)]
struct Token<'a> {
    text: &'a str,
    end: usize,
}

fn tokens(line: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut start = None;

    for (index, ch) in line.char_indices() {
        if ch.is_whitespace() {
            if let Some(token_start) = start.take() {
                tokens.push(Token {
                    text: &line[token_start..index],
                    end: index,
                });
            }
        } else if start.is_none() {
            start = Some(index);
        }
    }

    if let Some(token_start) = start {
        tokens.push(Token {
            text: &line[token_start..],
            end: line.len(),
        });
    }

    tokens
}

/// Remove entries for `id` from a `path.md` document.
///
/// A line is removed when its parsed ID matches. Prose lines are preserved.
pub fn remove(text: &str, id: u32) -> String {
    text.lines()
        .filter(|line| parse_entry(line).map(|e| e.id) != Some(id))
        .collect::<Vec<_>>()
        .join("\n")
        + if text.ends_with('\n') { "\n" } else { "" }
}

/// Append a canonical entry to a document that already has at least one
/// ranked reference. Empty/unranked path documents are left unchanged because
/// they mean the project has opted out of explicit ranking for now.
pub fn append_if_ranked(text: &str, id: u32, slug: &str) -> String {
    if references(text).is_empty() {
        return text.to_string();
    }

    let mut output = text.to_string();
    if !output.is_empty() && !output.ends_with('\n') {
        output.push('\n');
    }
    output.push_str(&format!("{id:04} {slug}\n"));
    output
}

/// Rewrite known entries to canonical `NNNN slug` form, preserving any
/// trailing free-form text. Unknown IDs and prose lines are preserved verbatim.
///
/// Pasted `clew list` lines are normalized only when the status-column shape is
/// unambiguous for the known slug (`NNNN status current-slug`).
pub fn normalize(text: &str, slugs_by_id: &BTreeMap<u32, String>) -> String {
    text.lines()
        .map(|line| normalize_line(line, slugs_by_id))
        .collect::<Vec<_>>()
        .join("\n")
        + if text.ends_with('\n') { "\n" } else { "" }
}

fn normalize_line(line: &str, slugs_by_id: &BTreeMap<u32, String>) -> String {
    let Some(entry) = parse_entry(line) else {
        return line.to_string();
    };
    let Some(canonical_slug) = slugs_by_id.get(&entry.id) else {
        return line.to_string();
    };

    let tokens = tokens(line);
    let slug_token_index = if has_status_column(line, canonical_slug) {
        2
    } else {
        1
    };
    let trailing = tokens
        .get(slug_token_index)
        .map(|token| &line[token.end..])
        .unwrap_or("");

    format!("{:04} {canonical_slug}{trailing}", entry.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn references_extracts_ids_and_ignores_prose() {
        let input = "# Path\n\nnotes\n0002 b\nnot-an-id line\n0042 forty-two\n";
        assert_eq!(references(input), vec![2, 42]);
    }

    #[test]
    fn references_ignores_old_hash_form() {
        let input = "- #0001-a\n- #0002-b\n";
        assert_eq!(references(input), Vec::<u32>::new());
    }

    #[test]
    fn parse_entry_extracts_id_and_slug() {
        let entry = parse_entry("0005 add-init-bootstrap").unwrap();
        assert_eq!(entry.id, 5);
        assert_eq!(entry.slug, Some("add-init-bootstrap"));
    }

    #[test]
    fn parse_entry_treats_status_word_as_slug_without_context() {
        let entry = parse_entry("0005 todo p0").unwrap();
        assert_eq!(entry.id, 5);
        assert_eq!(entry.slug, Some("todo"));
    }

    #[test]
    fn has_status_column_requires_known_slug_match() {
        assert!(has_status_column(
            "0005 in_progress add-init-bootstrap",
            "add-init-bootstrap"
        ));
        assert!(!has_status_column("0005 todo p0", "todo"));
    }

    #[test]
    fn has_status_column_prefers_canonical_when_slug_equals_status_word() {
        // Slug = `todo`, line is canonical `NNNN slug annotation`.
        assert!(!has_status_column("0001 todo todo", "todo"));
    }

    #[test]
    fn normalize_preserves_annotation_when_slug_equals_status_word() {
        let mut slugs = BTreeMap::new();
        slugs.insert(1, "todo".to_string());

        assert_eq!(normalize("0001 todo todo\n", &slugs), "0001 todo todo\n");
    }

    #[test]
    fn parse_entry_preserves_trailing_annotation() {
        let entry = parse_entry("0005 add-init-bootstrap // p0 note").unwrap();
        assert_eq!(entry.id, 5);
        assert_eq!(entry.slug, Some("add-init-bootstrap"));
    }

    #[test]
    fn parse_entry_allows_id_only() {
        let entry = parse_entry("0005").unwrap();
        assert_eq!(entry.id, 5);
        assert_eq!(entry.slug, None);
    }

    #[test]
    fn parse_entry_rejects_lines_without_separator_after_id() {
        assert_eq!(parse_entry("00051-foo"), None);
        assert_eq!(parse_entry("0005-foo"), None);
    }

    #[test]
    fn parse_entry_ignores_prose_lines() {
        assert_eq!(parse_entry("# Path"), None);
        assert_eq!(parse_entry(""), None);
        assert_eq!(parse_entry("priority list:"), None);
    }

    #[test]
    fn remove_drops_matching_entry_lines() {
        let input = "0001 a\n0002 b\n";
        assert_eq!(remove(input, 1), "0002 b\n");
    }

    #[test]
    fn remove_preserves_non_matching_text() {
        let input = "# Path\n\nnotes\n0002 b\n";
        assert_eq!(remove(input, 1), input);
    }

    #[test]
    fn append_if_ranked_appends_canonical_entry_when_path_has_references() {
        assert_eq!(append_if_ranked("0001 a\n", 2, "b"), "0001 a\n0002 b\n");
    }

    #[test]
    fn append_if_ranked_leaves_unranked_path_unchanged() {
        assert_eq!(append_if_ranked("# Path\n\n", 2, "b"), "# Path\n\n");
        assert_eq!(append_if_ranked("", 2, "b"), "");
    }

    #[test]
    fn normalize_rewrites_stale_slug_and_preserves_trailing_annotation() {
        let mut slugs = BTreeMap::new();
        slugs.insert(2, "current-slug".to_string());

        assert_eq!(
            normalize("0002 old-slug // note\n0003 missing\n", &slugs),
            "0002 current-slug // note\n0003 missing\n"
        );
    }

    #[test]
    fn normalize_leaves_canonical_entries_untouched() {
        let mut slugs = BTreeMap::new();
        slugs.insert(2, "stable".to_string());

        assert_eq!(normalize("0002 stable\n", &slugs), "0002 stable\n");
    }

    #[test]
    fn references_accept_pasted_list_output_ids() {
        let input = "0002 in_progress path-md\n0019 backlog     verify-clew\n";
        assert_eq!(references(input), vec![2, 19]);
    }

    #[test]
    fn normalize_drops_status_column_when_slug_matches() {
        let mut slugs = BTreeMap::new();
        slugs.insert(2, "current".to_string());

        assert_eq!(
            normalize("0002 in_progress current // note\n", &slugs),
            "0002 current // note\n"
        );
    }

    #[test]
    fn normalize_does_not_treat_status_word_slug_as_status_column() {
        let mut slugs = BTreeMap::new();
        slugs.insert(2, "todo".to_string());

        assert_eq!(normalize("0002 todo p0\n", &slugs), "0002 todo p0\n");
    }
}

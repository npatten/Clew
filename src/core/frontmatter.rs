use crate::core::increment::Increment;
use crate::error::ClewError;

#[derive(Debug)]
pub struct ParsedFile {
    pub increment: Increment,
    pub body: String,
}

/// Parse markdown text with YAML frontmatter into an Increment + body.
///
/// Expects the file to start with `---\n`. The YAML chunk between the two
/// `---` delimiters is deserialized into an `Increment`; everything after
/// the closing `---\n` is returned verbatim as `body`.
pub fn parse(text: &str) -> Result<ParsedFile, ClewError> {
    if !text.starts_with("---\n") {
        return Err(ClewError::Frontmatter(
            "file does not start with '---'".into(),
        ));
    }

    // Strip the leading `---\n` then split on the next `---\n`.
    let after_open = &text[4..];
    let close_pos = after_open.find("\n---\n").ok_or_else(|| {
        ClewError::Frontmatter("missing closing '---' delimiter".into())
    })?;

    let yaml_chunk = &after_open[..close_pos];
    // body starts after the `\n---\n` (5 chars)
    let body = after_open[close_pos + 5..].to_string();

    let increment: Increment = yaml_serde::from_str(yaml_chunk)
        .map_err(|e| ClewError::Frontmatter(e.to_string()))?;

    Ok(ParsedFile { increment, body })
}

/// Serialize an Increment + body back to a markdown file string.
pub fn serialize(file: &ParsedFile) -> Result<String, ClewError> {
    let yaml = yaml_serde::to_string(&file.increment)
        .map_err(|e| ClewError::Frontmatter(e.to_string()))?;

    Ok(format!("---\n{}---\n{}", yaml, file.body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use rstest::rstest;

    fn minimal_frontmatter(extra: &str) -> String {
        format!(
            "---\nid: 1\nstatus: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n{}---\n",
            extra
        )
    }

    #[test]
    fn round_trip_preserves_unknown_fields() {
        let input = "---\nid: 42\nstatus: todo\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\npriority: high\njira: PROJ-1234\n---\n# My increment\n\nSome body.\n";
        let parsed = parse(input).expect("parse should succeed");

        assert_eq!(parsed.increment.id, 42);

        let extra = &parsed.increment.extra;
        assert!(extra.contains_key("priority"), "priority field missing");
        assert!(extra.contains_key("jira"), "jira field missing");

        let serialized = serialize(&parsed).expect("serialize should succeed");
        let reparsed = parse(&serialized).expect("re-parse should succeed");

        assert_eq!(reparsed.increment.id, 42);
        assert!(reparsed.increment.extra.contains_key("priority"));
        assert!(reparsed.increment.extra.contains_key("jira"));
    }

    #[test]
    fn round_trip_preserves_body_verbatim() {
        let body = "\n# Title\n\n- [x] Done task\n- [ ] Pending task\n\n```rust\nfn main() {}\n```\n\nSome trailing prose.\n";
        let input = format!(
            "---\nid: 1\nstatus: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n---\n{}",
            body
        );

        let parsed = parse(&input).expect("parse should succeed");
        assert_eq!(parsed.body, body);

        let serialized = serialize(&parsed).expect("serialize should succeed");
        let reparsed = parse(&serialized).expect("re-parse should succeed");
        assert_eq!(reparsed.body, body);
    }

    #[test]
    fn missing_leading_delimiter_errors() {
        let input = "id: 1\nstatus: backlog\n---\n";
        let err = parse(input).expect_err("should fail without leading ---");
        assert!(matches!(err, ClewError::Frontmatter(_)));
    }

    #[test]
    fn missing_closing_delimiter_errors() {
        let input = "---\nid: 1\nstatus: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n";
        let err = parse(input).expect_err("should fail without closing ---");
        assert!(matches!(err, ClewError::Frontmatter(_)));
    }

    #[test]
    fn empty_body_parses_and_serializes() {
        let input = "---\nid: 1\nstatus: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n---\n";
        let parsed = parse(input).expect("parse should succeed");
        assert_eq!(parsed.body, "");

        let serialized = serialize(&parsed).expect("serialize should succeed");
        let reparsed = parse(&serialized).expect("re-parse should succeed");
        assert_eq!(reparsed.body, "");
    }

    #[rstest]
    #[case("id: 1\nstatus: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\n", "missing updated_at")]
    #[case("status: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n", "missing id")]
    #[case("id: 1\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n", "missing status")]
    fn required_fields_missing_errors(#[case] yaml: &str, #[case] _desc: &str) {
        let input = format!("---\n{}---\n", yaml);
        parse(&input).expect_err("should fail on missing required field");
    }

    #[test]
    fn unknown_status_value_errors() {
        let input = "---\nid: 1\nstatus: flying\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n---\n";
        let err = parse(input).expect_err("should fail on unknown status");
        assert!(matches!(err, ClewError::Frontmatter(_)));
    }

    #[test]
    fn parent_is_optional() {
        let input = minimal_frontmatter("");
        let parsed = parse(&input).expect("parse should succeed without parent");
        assert!(parsed.increment.parent.is_none());

        let with_parent = minimal_frontmatter("parent: 7\n");
        let parsed2 = parse(&with_parent).expect("parse should succeed with parent");
        assert_eq!(parsed2.increment.parent, Some(7));
    }

    #[test]
    fn tags_optional_absent() {
        let input = minimal_frontmatter("");
        let parsed = parse(&input).expect("parse should succeed without tags");
        assert!(parsed.increment.tags.is_empty());
    }

    #[test]
    fn tags_optional_present() {
        let input = minimal_frontmatter("tags: [auth, p0]\n");
        let parsed = parse(&input).expect("parse should succeed with tags");
        assert_eq!(parsed.increment.tags, vec!["auth", "p0"]);
    }

    #[test]
    fn all_statuses_parse() {
        use crate::core::increment::Status;
        for (s, expected) in [
            ("backlog", Status::Backlog),
            ("todo", Status::Todo),
            ("in_progress", Status::InProgress),
            ("done", Status::Done),
            ("abandoned", Status::Abandoned),
        ] {
            let input = format!(
                "---\nid: 1\nstatus: {}\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n---\n",
                s
            );
            let parsed = parse(&input).expect("should parse");
            assert_eq!(parsed.increment.status, expected);
        }
    }

    #[test]
    fn timestamps_parse_correctly() {
        let input = minimal_frontmatter("");
        let parsed = parse(&input).expect("parse should succeed");
        let expected: DateTime<chrono::Utc> = "2026-04-26T10:00:00Z".parse().unwrap();
        assert_eq!(parsed.increment.created_at, expected);
        assert_eq!(parsed.increment.updated_at, expected);
    }

    #[test]
    fn snapshot_round_trip() {
        // NOTE: YAML treats bare `#` after a space as a comment — reference values
        // like `blocked_reason: waiting on #0039` must be quoted in frontmatter.
        let input = "---\nid: 42\nstatus: in_progress\nparent: 7\nblocked_reason: \"waiting on #0039\"\ntags:\n- auth\n- p0\ncreated_at: \"2026-04-20T10:00:00Z\"\nupdated_at: \"2026-04-25T14:30:00Z\"\npriority: high\njira: PROJ-1234\n---\n\n# Add OAuth routes\n\n- [x] Scaffold handlers\n- [ ] Write tests\n";
        let parsed = parse(input).expect("parse should succeed");
        let serialized = serialize(&parsed).expect("serialize should succeed");
        insta::assert_snapshot!(serialized);
    }
}

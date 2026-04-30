use crate::core::increment::Increment;
use crate::error::ClewError;

#[derive(Debug)]
pub struct ParsedFile {
    pub increment: Increment,
    pub body: String,
}

/// Parse markdown text with YAML frontmatter into an Increment + body.
///
/// Expects the file to start with `---` followed by LF or CRLF. The YAML chunk
/// between the two `---` delimiters is deserialized into an `Increment`;
/// everything after the closing delimiter is returned verbatim as `body`.
/// CRLF is accepted in frontmatter input, but serialization normalizes
/// frontmatter delimiters and YAML to LF. Body content remains verbatim.
pub fn parse(text: &str) -> Result<ParsedFile, ClewError> {
    let (after_open, close_delimiter) = if let Some(rest) = text.strip_prefix("---\n") {
        (rest, "\n---\n")
    } else if let Some(rest) = text.strip_prefix("---\r\n") {
        (rest, "\r\n---\r\n")
    } else {
        return Err(ClewError::Frontmatter(
            "file does not start with '---'".into(),
        ));
    };

    let close_pos = after_open
        .find(close_delimiter)
        .ok_or_else(|| ClewError::Frontmatter("missing closing '---' delimiter".into()))?;

    let yaml_chunk = after_open[..close_pos].replace("\r\n", "\n");
    let body = after_open[close_pos + close_delimiter.len()..].to_string();

    let increment: Increment =
        yaml_serde::from_str(&yaml_chunk).map_err(|e| ClewError::Frontmatter(e.to_string()))?;

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
        // Covers: scalar string, scalar number, bool, string-with-`#` (must be quoted),
        // nested map, array of strings, array of mixed scalars.
        let input = "---\n\
id: 42\n\
status: todo\n\
created_at: \"2026-04-26T10:00:00Z\"\n\
updated_at: \"2026-04-26T10:00:00Z\"\n\
priority: high\n\
story_points: 5\n\
needs_review: true\n\
linked_issue: \"see #0039\"\n\
estimates:\n  best: 1\n  worst: 5\n\
reviewers:\n- alice\n- bob\n\
mixed:\n- 1\n- two\n- true\n\
---\n# My increment\n\nSome body.\n";
        let parsed = parse(input).expect("parse should succeed");

        assert_eq!(parsed.increment.id, 42);

        // All unknown keys must survive parse.
        for key in [
            "priority",
            "story_points",
            "needs_review",
            "linked_issue",
            "estimates",
            "reviewers",
            "mixed",
        ] {
            assert!(
                parsed.increment.extra.contains_key(key),
                "unknown field '{key}' missing after parse"
            );
        }

        // Round-trip equality on the entire extra map: values, types, nested
        // structure, all preserved (not just key presence).
        let serialized = serialize(&parsed).expect("serialize should succeed");
        let reparsed = parse(&serialized).expect("re-parse should succeed");
        assert_eq!(
            parsed.increment.extra, reparsed.increment.extra,
            "extra map drifted across round-trip"
        );
    }

    #[test]
    fn unknown_field_serialization_is_deterministic() {
        // Two parses of the same input must serialize identically — protects
        // against HashMap-style nondeterminism in the extra map.
        let input = "---\nid: 1\nstatus: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\nzeta: 1\nalpha: 2\nmu: 3\nbeta: 4\n---\n";
        let a = serialize(&parse(input).unwrap()).unwrap();
        let b = serialize(&parse(input).unwrap()).unwrap();
        assert_eq!(a, b, "serialization is nondeterministic across parses");
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
    fn crlf_frontmatter_parses_and_serializes_as_lf_without_touching_body() {
        let body = "\r\n# Title\r\n\r\nBody line\r\n";
        let input = format!(
            "---\r\nid: 1\r\nstatus: backlog\r\ncreated_at: \"2026-04-26T10:00:00Z\"\r\nupdated_at: \"2026-04-26T10:00:00Z\"\r\n---\r\n{body}"
        );

        let parsed = parse(&input).expect("parse should succeed");
        assert_eq!(parsed.body, body);

        let serialized = serialize(&parsed).expect("serialize should succeed");
        let serialized_frontmatter = serialized.split_once(body).unwrap().0;
        assert!(!serialized_frontmatter.contains('\r'));
        assert_eq!(parse(&serialized).unwrap().body, body);
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
    #[case(
        "id: 1\nstatus: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\n",
        "missing updated_at"
    )]
    #[case("status: backlog\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n", "missing id")]
    #[case(
        "id: 1\ncreated_at: \"2026-04-26T10:00:00Z\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n",
        "missing status"
    )]
    #[case(
        "id: 1\nstatus: backlog\nupdated_at: \"2026-04-26T10:00:00Z\"\n",
        "missing created_at"
    )]
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

    #[rstest]
    #[case("2026-04-26T12:00:00+02:00", "non-UTC offset")]
    #[case("2026-04-26T10:00:00.123Z", "subsecond precision")]
    #[case("2026-04-26T10:00:00", "missing Z suffix")]
    fn invalid_timestamp_formats_error(#[case] timestamp: &str, #[case] _desc: &str) {
        let input = format!(
            "---\nid: 1\nstatus: backlog\ncreated_at: \"{}\"\nupdated_at: \"2026-04-26T10:00:00Z\"\n---\n",
            timestamp
        );
        let err = parse(&input).expect_err("should reject invalid timestamp format");
        assert!(matches!(err, ClewError::Frontmatter(_)));
    }

    #[test]
    fn timestamp_serialization_uses_utc_second_precision() {
        let input = minimal_frontmatter("");
        let serialized = serialize(&parse(&input).unwrap()).unwrap();

        assert!(serialized.contains("created_at: 2026-04-26T10:00:00Z"));
        assert!(serialized.contains("updated_at: 2026-04-26T10:00:00Z"));
        assert!(!serialized.contains(".000"));
        assert!(!serialized.contains("+00:00"));
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

use std::collections::HashSet;

/// Validate a Clew tag.
///
/// Tags are preserved exactly once valid; invalid input is rejected rather than
/// normalized so case-sensitive duplicates cannot drift into frontmatter.
pub fn validate(tag: &str) -> Result<(), InvalidTag> {
    let mut chars = tag.chars();
    let Some(first) = chars.next() else {
        return Err(InvalidTag::new(tag));
    };

    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return Err(InvalidTag::new(tag));
    }

    if chars.any(|c| !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '-') {
        return Err(InvalidTag::new(tag));
    }

    Ok(())
}

pub fn validate_all<'a>(tags: impl IntoIterator<Item = &'a String>) -> Result<(), InvalidTag> {
    for tag in tags {
        validate(tag)?;
    }
    Ok(())
}

pub fn dedupe_preserving_order(tags: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for tag in tags {
        if seen.insert(tag.clone()) {
            deduped.push(tag);
        }
    }
    deduped
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidTag {
    pub value: String,
    pub hint: Option<String>,
}

impl InvalidTag {
    fn new(value: &str) -> Self {
        let hint = suggested_tag(value).filter(|suggestion| suggestion != value);
        Self {
            value: value.to_string(),
            hint,
        }
    }
}

fn suggested_tag(value: &str) -> Option<String> {
    let mut suggestion = String::new();
    let mut last_was_dash = false;

    for c in value.trim().trim_start_matches('#').chars() {
        let c = c.to_ascii_lowercase();
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            suggestion.push(c);
            last_was_dash = false;
        } else if matches!(c, '-' | '_' | ',' | ' ' | '\t')
            && !suggestion.is_empty()
            && !last_was_dash
        {
            suggestion.push('-');
            last_was_dash = true;
        }
    }

    while suggestion.ends_with('-') {
        suggestion.pop();
    }

    if suggestion.is_empty() {
        None
    } else {
        Some(suggestion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_supported_grammar() {
        for tag in ["windows", "p0", "area-cli", "a1-b2"] {
            validate(tag).unwrap();
        }
    }

    #[test]
    fn rejects_invalid_tags_with_hints() {
        assert_eq!(
            validate("Windows").unwrap_err().hint,
            Some("windows".into())
        );
        assert_eq!(validate("#p0").unwrap_err().hint, Some("p0".into()));
        assert_eq!(
            validate("windows,distribution").unwrap_err().hint,
            Some("windows-distribution".into())
        );
        assert_eq!(validate("").unwrap_err().hint, None);
    }

    #[test]
    fn dedupes_preserving_first_seen_order() {
        assert_eq!(
            dedupe_preserving_order(vec!["a".into(), "b".into(), "a".into()]),
            vec!["a".to_string(), "b".to_string()]
        );
    }
}

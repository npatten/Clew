use std::collections::BTreeMap;

/// Remove entries for `id` from a path.md document.
///
/// The parser is intentionally permissive: if a line contains a `#NNNN`
/// reference matching `id`, that whole line is removed. Everything else is
/// preserved verbatim.
pub fn remove(text: &str, id: u32) -> String {
    let needle = format!("#{id:04}");
    text.lines()
        .filter(|line| !line.contains(&needle))
        .collect::<Vec<_>>()
        .join("\n")
        + if text.ends_with('\n') { "\n" } else { "" }
}

/// Normalize known `#NNNN[-slug]` references to `#NNNN-current-slug`.
/// Unknown references and surrounding annotations are preserved.
pub fn normalize(text: &str, slugs_by_id: &BTreeMap<u32, String>) -> String {
    text.lines()
        .map(|line| normalize_line(line, slugs_by_id))
        .collect::<Vec<_>>()
        .join("\n")
        + if text.ends_with('\n') { "\n" } else { "" }
}

fn normalize_line(line: &str, slugs_by_id: &BTreeMap<u32, String>) -> String {
    let Some(hash) = line.find('#') else {
        return line.to_string();
    };
    let id_start = hash + 1;
    let id_end = id_start + 4;
    let Some(id_text) = line.get(id_start..id_end) else {
        return line.to_string();
    };
    if !id_text.chars().all(|c| c.is_ascii_digit()) {
        return line.to_string();
    }
    let Ok(id) = id_text.parse::<u32>() else {
        return line.to_string();
    };
    let Some(slug) = slugs_by_id.get(&id) else {
        return line.to_string();
    };

    let mut ref_end = id_end;
    if line[id_end..].starts_with('-') {
        ref_end += 1;
        for ch in line[ref_end..].chars() {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ref_end += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    format!("{}#{id:04}-{}{}", &line[..hash], slug, &line[ref_end..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_drops_matching_reference_lines() {
        let input = "# Path\n\n- #0001-a\n- #0002-b\n";
        assert_eq!(remove(input, 1), "# Path\n\n- #0002-b\n");
    }

    #[test]
    fn remove_preserves_non_matching_text() {
        let input = "# Path\n\nnotes\n- #0002-b\n";
        assert_eq!(remove(input, 1), input);
    }

    #[test]
    fn normalize_rewrites_known_references_and_preserves_annotations() {
        let mut slugs = BTreeMap::new();
        slugs.insert(2, "current-slug".to_string());

        assert_eq!(
            normalize("- #0002-old-slug // note\n- #0003-missing\n", &slugs),
            "- #0002-current-slug // note\n- #0003-missing\n"
        );
    }
}

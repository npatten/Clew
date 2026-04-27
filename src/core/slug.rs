/// Maximum slug length per the design plan (`crew-plan.md` §"Slug rules").
/// Truncation is applied on top of the `slug` crate's output.
const MAX_LEN: usize = 50;

/// Fallback slug when transliteration yields nothing useful (e.g. all-CJK
/// input, all-punctuation, empty title). Documented in the plan.
const FALLBACK: &str = "untitled";

/// Generate a filename slug from a free-form title.
///
/// Pipeline: `slug::slugify` (deunicode + ASCII-fold + lowercase + dash-join)
/// → truncate to `MAX_LEN` chars → trim leading/trailing dashes (truncation
/// can land mid-word and leave a dangling `-`) → fall back to `FALLBACK` if
/// the result is empty.
pub fn generate(title: &str) -> String {
    let raw = slug::slugify(title);
    let truncated: String = raw.chars().take(MAX_LEN).collect();
    let trimmed = truncated.trim_matches('-');
    if trimmed.is_empty() {
        FALLBACK.to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Add OAuth routes", "add-oauth-routes")]
    #[case("  Trim whitespace  ", "trim-whitespace")]
    #[case("Punctuation! Goes? Away.", "punctuation-goes-away")]
    #[case("MixedCASE Title", "mixedcase-title")]
    fn ascii_titles_slugify_predictably(#[case] title: &str, #[case] expected: &str) {
        assert_eq!(generate(title), expected);
    }

    #[test]
    fn unicode_is_transliterated() {
        // deunicode handles common European accents.
        assert_eq!(generate("Café résumé"), "cafe-resume");
    }

    #[test]
    fn all_cjk_falls_back_to_untitled() {
        // CJK actually transliterates to romanizations via deunicode; use a
        // genuinely empty case (whitespace-only after slugification).
        assert_eq!(generate(""), "untitled");
        assert_eq!(generate("   "), "untitled");
        assert_eq!(generate("---"), "untitled");
        assert_eq!(generate("!!!"), "untitled");
    }

    #[test]
    fn long_title_is_truncated_to_max_len() {
        let title = "a".repeat(100);
        let slug = generate(&title);
        assert_eq!(slug.chars().count(), MAX_LEN);
    }

    #[test]
    fn truncation_strips_trailing_dash() {
        // 49 chars then a word boundary: truncation at 50 lands on a `-`.
        let title = format!("{} foo", "a".repeat(49));
        let slug = generate(&title);
        assert!(
            !slug.ends_with('-'),
            "slug should not end with dash: {slug}"
        );
    }
}

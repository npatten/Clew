use crate::core::tag;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClewError {
    #[error("increment not found: {0}")]
    NotFound(String),

    #[error(
        "not inside a clew project (no .clew/ directory found in this directory or any parent)"
    )]
    ClewRootNotFound,

    #[error("invalid status transition: {from} → {to}")]
    InvalidTransition { from: String, to: String },

    #[error("slug '{slug}' is already used by #{existing}\n       try a more specific title")]
    SlugCollision { slug: String, existing: String },

    #[error("frontmatter parse error: {0}")]
    Frontmatter(String),

    #[error("stdin appears to contain frontmatter (starts with `---`).\n       `clew new` writes frontmatter itself; pass body content only.")]
    InvalidStdin,

    #[error(
        "invalid --status value: '{0}' (expected: backlog, todo, in_progress, done, abandoned)"
    )]
    InvalidStatusFilter(String),

    #[error("invalid tag: '{value}' (expected [a-z0-9][a-z0-9-]*){hint}", hint = hint.as_ref().map(|h| format!("; try: {h}")).unwrap_or_default())]
    InvalidTag { value: String, hint: Option<String> },

    #[error("increment #{id:04} does not have tag '{tag}'")]
    MissingTag { id: u32, tag: String },

    #[error("reason must not be empty")]
    EmptyReason,

    #[error("cannot {action} archived increment #{id:04}; reopen it first")]
    ArchivedIncrement { action: &'static str, id: u32 },

    #[error("no todo increments found")]
    NoNextIncrement,

    #[error("lint found {0} issue(s)")]
    LintFailed(usize),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("not yet implemented")]
    Unimplemented,
}

impl From<tag::InvalidTag> for ClewError {
    fn from(value: tag::InvalidTag) -> Self {
        ClewError::InvalidTag {
            value: value.value,
            hint: value.hint,
        }
    }
}

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

    #[error(
        "invalid --status value: '{0}' (expected: backlog, todo, in_progress, done, abandoned)"
    )]
    InvalidStatusFilter(String),

    #[error("reason must not be empty")]
    EmptyReason,

    #[error("cannot {action} archived increment #{id:04}; reopen it first")]
    ArchivedIncrement { action: &'static str, id: u32 },

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("not yet implemented")]
    Unimplemented,
}

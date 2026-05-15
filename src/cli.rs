use crate::error::ClewError;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "clew",
    version,
    about = "Lightweight git-native project management"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize .clew/ in the current directory
    Init,
    /// Create a new increment; reads body content from non-TTY stdin
    #[command(after_help = r#"Examples:
  clew new "Add OAuth route handlers"
  clew new --ready "Add OAuth route handlers"
  clew new "Add OAuth route handlers" < body.md
  printf 'Body text here' | clew new "Add OAuth route handlers"

If stdin is non-interactive, it is read verbatim as the increment body."#)]
    New {
        /// Increment title
        title: String,
        /// Create the increment as todo instead of backlog
        #[arg(long)]
        ready: bool,
        /// Parent increment ID
        #[arg(long)]
        parent: Option<u32>,
        /// Tag to attach; repeat for multiple tags
        #[arg(long = "tag")]
        tags: Vec<String>,
    },
    /// Show an increment by ID or slug
    #[command(alias = "view")]
    Show { id: String },
    /// List increments
    List {
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(short = 'a', long)]
        all: bool,
    },
    /// Mark an increment as in_progress
    Start { id: String },
    /// Block an increment with a reason
    Block { id: String, reason: String },
    /// Unblock an increment
    Unblock { id: String },
    /// Add one or more tags to an increment
    Tag {
        id: String,
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// Remove one or more tags from an increment
    Untag {
        id: String,
        #[arg(required = true)]
        tags: Vec<String>,
    },
    /// Mark an increment as done and archive it
    Done { id: String },
    /// Abandon an increment with an optional reason
    Abandon { id: String, reason: Option<String> },
    /// Reopen an archived increment
    Reopen { id: String },
    /// Show or start the next increment
    Next {
        #[arg(long)]
        start: bool,
    },
    /// Open path.md in your editor
    Path,
    /// Check for drift and dangling references
    Lint,
    /// Atomically renumber an increment ID
    Renumber { old: u32, new: u32 },
}

impl Cli {
    pub fn dispatch(self) -> Result<(), ClewError> {
        match self.command {
            None => {
                eprintln!("Run `clew --help` for usage.");
                Ok(())
            }
            Some(Command::Init) => crate::commands::init::run(),
            Some(Command::New {
                title,
                ready,
                parent,
                tags,
            }) => crate::commands::new::run(&title, ready, parent, &tags),
            Some(Command::Show { id }) => crate::commands::show::run(&id),
            Some(Command::List { tag, status, all }) => {
                crate::commands::list::run(tag.as_deref(), status.as_deref(), all)
            }
            Some(Command::Start { id }) => crate::commands::start::run(&id),
            Some(Command::Block { id, reason }) => crate::commands::block::run(&id, &reason),
            Some(Command::Unblock { id }) => crate::commands::unblock::run(&id),
            Some(Command::Tag { id, tags }) => crate::commands::tag::run(&id, &tags),
            Some(Command::Untag { id, tags }) => crate::commands::untag::run(&id, &tags),
            Some(Command::Done { id }) => crate::commands::done::run(&id),
            Some(Command::Abandon { id, reason }) => {
                crate::commands::abandon::run(&id, reason.as_deref())
            }
            Some(Command::Reopen { id }) => crate::commands::reopen::run(&id),
            Some(Command::Next { start }) => crate::commands::next::run(start),
            Some(Command::Lint) => crate::commands::lint::run(),
            Some(_) => Err(ClewError::Unimplemented),
        }
    }
}

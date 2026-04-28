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
    /// Create a new increment
    New {
        /// Increment title
        title: String,
        /// Create the increment as todo instead of backlog
        #[arg(long)]
        ready: bool,
        /// Parent increment ID
        #[arg(long)]
        parent: Option<u32>,
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
        #[arg(long)]
        all: bool,
    },
    /// Promote an increment from backlog to todo
    Promote { id: u32 },
    /// Mark an increment as in_progress
    Start { id: String },
    /// Block an increment with a reason
    Block { id: String, reason: String },
    /// Unblock an increment
    Unblock { id: String },
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
    /// Open relay.md in your editor
    Relay,
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
            }) => crate::commands::new::run(&title, ready, parent),
            Some(Command::Show { id }) => crate::commands::show::run(&id),
            Some(Command::List { tag, status, all }) => {
                crate::commands::list::run(tag.as_deref(), status.as_deref(), all)
            }
            Some(Command::Start { id }) => crate::commands::start::run(&id),
            Some(Command::Block { id, reason }) => crate::commands::block::run(&id, &reason),
            Some(Command::Unblock { id }) => crate::commands::unblock::run(&id),
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

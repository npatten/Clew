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
        title: String,
        #[arg(long)]
        ready: bool,
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
    Start { id: u32 },
    /// Block an increment with a reason
    Block { id: u32, reason: String },
    /// Unblock an increment
    Unblock { id: u32 },
    /// Mark an increment as done and archive it
    Done { id: u32 },
    /// Abandon an increment with a reason
    Abandon { id: u32, reason: String },
    /// Reopen an archived increment
    Reopen { id: u32 },
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
            Some(Command::New {
                title,
                ready,
                parent,
            }) => crate::commands::new::run(&title, ready, parent),
            Some(Command::Show { id }) => crate::commands::show::run(&id),
            Some(Command::List { tag, status, all }) => {
                crate::commands::list::run(tag.as_deref(), status.as_deref(), all)
            }
            Some(Command::Start { .. }) => crate::commands::start::run(),
            Some(Command::Done { .. }) => crate::commands::done::run(),
            Some(Command::Next { .. }) => crate::commands::next::run(),
            Some(_) => Err(ClewError::Unimplemented),
        }
    }
}

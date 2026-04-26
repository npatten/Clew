pub mod cli;
pub mod commands;
pub mod core;
pub mod error;
pub mod storage;

pub use error::ClewError;

pub fn run() -> Result<(), ClewError> {
    use clap::Parser;
    let cli = cli::Cli::parse();
    cli.dispatch()
}

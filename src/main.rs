use std::process::ExitCode;

fn main() -> ExitCode {
    match clew::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            match &e {
                clew::ClewError::NotFound(_)
                | clew::ClewError::ClewRootNotFound
                | clew::ClewError::InvalidTransition { .. }
                | clew::ClewError::SlugCollision { .. }
                | clew::ClewError::InvalidStatusFilter(_)
                | clew::ClewError::EmptyReason
                | clew::ClewError::ArchivedIncrement { .. }
                | clew::ClewError::NoNextIncrement
                | clew::ClewError::LintFailed(_)
                | clew::ClewError::Unimplemented => ExitCode::from(1),
                clew::ClewError::Io(_) | clew::ClewError::Frontmatter(_) => ExitCode::from(2),
            }
        }
    }
}

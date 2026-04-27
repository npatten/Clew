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
                | clew::ClewError::Unimplemented => ExitCode::from(1),
                clew::ClewError::Io(_) | clew::ClewError::Frontmatter(_) => ExitCode::from(2),
            }
        }
    }
}

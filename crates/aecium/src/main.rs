mod cli;
mod lex;
mod token;
mod util;

use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::cli() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

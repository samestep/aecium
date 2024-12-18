mod cli;
mod encoding;
mod name;
mod path;
mod source;
mod syntax;

fn main() -> std::process::ExitCode {
    cli::cli()
}

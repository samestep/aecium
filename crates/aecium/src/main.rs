mod cli;
mod encoding;
mod name;
mod path;
mod scope;
mod source;
mod syntax;

fn main() -> std::process::ExitCode {
    cli::cli()
}

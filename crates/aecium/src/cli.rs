use std::process::ExitCode;

use clap::Parser;

use crate::syntax::Tree;

#[derive(Debug, Parser)]
struct Cli {
    /// Path to the crate root.
    root: String,

    #[clap(long)]
    edition: ra_ap_parser::Edition,
}

fn cli_result() -> Result<(), ExitCode> {
    let args = Cli::parse();
    let mut tree = Tree::new(args.edition, &args.root).map_err(|e| {
        eprintln!("failed to read crate root: {e}");
        ExitCode::FAILURE
    })?;
    tree.expand().map_err(|e| {
        eprintln!("failed to expand crate: {e}");
        ExitCode::FAILURE
    })?;
    tree.print();
    Ok(())
}

pub fn cli() -> ExitCode {
    match cli_result() {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code,
    }
}

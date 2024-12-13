use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    file: PathBuf,

    #[clap(long)]
    edition: ra_ap_parser::Edition,
}

fn cli_result() -> Result<(), ExitCode> {
    let args = Cli::parse();
    let text = fs::read_to_string(&args.file).map_err(|e| {
        eprintln!("failed to read {:?}: {e}", args.file);
        ExitCode::FAILURE
    })?;
    let lexed = ra_ap_parser::LexedStr::new(args.edition, &text);
    let input = lexed.to_input(args.edition);
    let output = ra_ap_parser::TopEntryPoint::SourceFile.parse(&input, args.edition);
    for step in output.iter() {
        println!("{:?}", step);
    }
    Ok(())
}

pub fn cli() -> ExitCode {
    match cli_result() {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code,
    }
}

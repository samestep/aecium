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
    let mut i: usize = 0;
    let mut d: usize = 0;
    for step in output.iter() {
        if let ra_ap_parser::Step::Exit = step {
            d -= 1;
        }
        for _ in 0..d {
            print!("  ");
        }
        match step {
            ra_ap_parser::Step::Token {
                kind,
                n_input_tokens,
            } => {
                while lexed.kind(i).is_trivia() {
                    i += 1;
                }
                let n = usize::from(n_input_tokens);
                println!("{kind:?} {:?}", lexed.range_text(i..i + n));
                i += n;
            }
            ra_ap_parser::Step::FloatSplit { .. } => todo!(),
            ra_ap_parser::Step::Enter { kind } => {
                println!("{kind:?} {{");
                d += 1;
            }
            ra_ap_parser::Step::Exit => println!("}}"),
            ra_ap_parser::Step::Error { msg } => println!("{msg:?}"),
        }
    }
    Ok(())
}

pub fn cli() -> ExitCode {
    match cli_result() {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code,
    }
}

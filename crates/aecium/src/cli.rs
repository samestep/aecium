use std::{fs, path::PathBuf};

use clap::Parser;

use crate::lex::lex;

#[derive(Debug, Parser)]
struct Cli {
    file: PathBuf,
}

pub fn cli() -> Result<(), ()> {
    let args = Cli::parse();
    let source = fs::read_to_string(&args.file).map_err(|e| {
        eprintln!("failed to read {:?}: {e}", args.file);
    })?;
    let tokens = lex(&source).map_err(|e| {
        eprintln!(
            "failed to lex {:?}: {} at byte range {:?}",
            args.file,
            e.message(),
            e.byte_range(),
        );
    })?;
    for token in tokens.iter() {
        println!("{:?}", token.kind);
    }
    Ok(())
}

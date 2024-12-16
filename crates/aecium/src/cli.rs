use std::{
    fs,
    path::PathBuf,
    process::ExitCode,
    time::{Duration, Instant},
};

use clap::Parser;

fn repeat<T>(n: usize, mut f: impl FnMut() -> T) -> (Duration, T) {
    let mut i = 0;
    let start = Instant::now();
    loop {
        let x = f();
        if i < n {
            i += 1;
        } else {
            return (start.elapsed(), x);
        }
    }
}

// https://github.com/gradbench/gradbench/blob/3fc7f9677297387af0742327500dfa2b3cfce044/crates/gradbench/src/cli.rs#L317-L331
fn duration_string(duration: Duration) -> String {
    let ms = duration.as_millis();
    let sec = ms / 1000;
    let min = sec / 60;
    if sec == 0 {
        format!("{:2} {:2} {:3}ms", "", "", ms)
    } else if min == 0 {
        format!("{:2} {:2}.{:03} s", "", sec, ms % 1000)
    } else if min < 60 {
        format!("{:2}:{:02}.{:03}  ", min, sec % 60, ms % 1000)
    } else {
        format!("{:2} {:2}>{:3}hr", "", "", " 1 ")
    }
}

#[derive(Debug, Parser)]
struct Cli {
    files: Vec<PathBuf>,

    #[clap(long)]
    edition: ra_ap_parser::Edition,

    #[clap(short)]
    n: usize,
}

fn cli_result() -> Result<(), ExitCode> {
    let Cli { files, edition, n } = Cli::parse();
    let mut lexing = Duration::default();
    let mut converting = Duration::default();
    let mut parsing = Duration::default();
    let mut bytes = 0;
    let mut tokens = 0;
    let mut bytes_trivia = 0;
    let mut tokens_trivia = 0;
    let mut parse_output_steps = 0;
    for file in files {
        let text = fs::read_to_string(&file).map_err(|e| {
            eprintln!("failed to read {file:?}: {e}");
            ExitCode::FAILURE
        })?;
        println!("{n} × parsing {} ...", file.display());

        let (t_lex, lexed) = repeat(n, || ra_ap_parser::LexedStr::new(edition, &text));
        lexing += t_lex;
        bytes += text.len();
        tokens += lexed.len();
        bytes_trivia += (0..lexed.len())
            .filter(|&i| lexed.kind(i).is_trivia())
            .map(|i| lexed.text_range(i).len())
            .sum::<usize>();
        tokens_trivia += (0..lexed.len())
            .filter(|&i| lexed.kind(i).is_trivia())
            .count();

        let (t_convert, input) = repeat(n, || lexed.to_input(edition));
        converting += t_convert;

        let (t_parse, output) = repeat(n, || {
            ra_ap_parser::TopEntryPoint::SourceFile.parse(&input, edition)
        });
        parsing += t_parse;
        parse_output_steps += output.iter().count();
    }

    println!();
    println!("{n} × lex     = {}", duration_string(lexing));
    println!("{n} × convert = {}", duration_string(converting));
    println!("{n} × parse   = {}", duration_string(parsing));

    println!();
    println!("{bytes} bytes");
    println!("{bytes_trivia} whitespace or comment bytes");
    println!("{tokens} tokens");
    println!("{tokens_trivia} whitespace or comment tokens");
    println!("{parse_output_steps} parse output steps");

    println!();
    let total = lexing + converting + parsing;
    let mb_s = (((n * bytes) as f64) / 1_000_000.) / total.as_secs_f64();
    let mt_s = (((n * (tokens - tokens_trivia)) as f64) / 1_000_000.) / total.as_secs_f64();
    println!("bytes:  {mb_s:.3} M/s     (including trivia)",);
    println!("tokens: {mt_s:.3} M/s (not including trivia)");

    Ok(())
}

pub fn cli() -> ExitCode {
    match cli_result() {
        Ok(()) => ExitCode::SUCCESS,
        Err(code) => code,
    }
}

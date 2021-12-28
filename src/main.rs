#![feature(stdin_forwarders)]

use clap::{Parser, ValueHint};

use std::{io, path::PathBuf, process};

#[derive(Parser, Debug)]
struct Args {
    /// Path to precessed by efm-langserver file
    #[clap(value_hint = ValueHint::FilePath)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    io::stdin()
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let (file, _) = line.split_once(':')?;
            if args.path.ends_with(file) {
                Some(line)
            } else {
                None
            }
        })
        .for_each(|line| eprintln!("{}", line));

    // efm-langserver expects linters to exit with code != 0
    process::exit(1)
}

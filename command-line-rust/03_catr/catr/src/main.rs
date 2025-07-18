use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
    /// Rust version of `cat`
struct Args {
    /// Files that should be read and concatenated [default: -]
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(
        short('n'), 
        long("number"), 
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// Number non-blank output lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        println!("{filename}");
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

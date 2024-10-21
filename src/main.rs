use std::io::{self, Read, Write};

use anyhow::Result;
use clap::Parser;

use convfmt::{dump_value, load_input, Format};

#[derive(Parser, Debug)]
#[command(about, version, author)]
struct CliArgs {
    #[arg(short, long, value_enum)]
    from: Format,

    #[arg(short, long, value_enum)]
    to: Format,

    #[arg(short, long)]
    compact: bool,
}

fn run_app() -> Result<()> {
    let args = CliArgs::parse();
    let input = read_input()?;
    let value = load_input(&input, args.from)?;
    let output = dump_value(&value, args.to, args.compact)?;
    write_output(&output)?;
    Ok(())
}

fn main() {
    if let Err(err) = run_app() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

fn read_input() -> Result<Vec<u8>> {
    let mut buf = vec![];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_output(output: &[u8]) -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(output)?;
    Ok(())
}

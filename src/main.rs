use std::io::{self, Read, Write};

use anyhow::{Context, Result};
use clap::Parser;

use convfmt::{Format, dump_value, ignore_unsupported, load_input, sort_keys};

#[derive(Parser, Debug)]
#[command(about, version, author)]
struct CliArgs {
    #[arg(short, long, value_enum)]
    from: Format,

    #[arg(short, long, value_enum)]
    to: Format,

    #[arg(short, long)]
    /// Compress output if possible (default = false)
    compact: bool,

    #[arg(short, long)]
    /// Sort keys of objects (default = false)
    sort_keys: bool,

    #[arg(short, long)]
    /// Drop values the target format can't represent, e.g. `null` for toml (default = false)
    ignore_unsupported: bool,
}

fn run_app() -> Result<()> {
    let args = CliArgs::parse();
    let input = read_input()?;
    let mut value = load_input(&input, args.from)?;
    if args.sort_keys {
        sort_keys(&mut value);
    }
    if args.ignore_unsupported {
        let skipped = ignore_unsupported(&mut value, args.to);
        if skipped > 0 {
            eprintln!(
                "Warning: skipped {skipped} value(s) unsupported by {}",
                args.to
            );
        }
    }
    let output = dump_value(&value, args.to, args.compact).with_context(|| {
        if args.ignore_unsupported || args.to.supports_null() {
            format!("can't dump to {}", args.to)
        } else {
            format!(
                "can't dump to {}: it has no representation for `null`, \
                 try --ignore-unsupported to drop such values",
                args.to
            )
        }
    })?;
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

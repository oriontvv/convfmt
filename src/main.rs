use std::io::{self, BufReader, BufWriter, Read, Write};

use anyhow::Result;
use clap::Parser;
use serde::Serialize;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CliArgs {
    #[clap(short, long, value_enum)]
    from: Format,

    #[clap(short, long, value_enum)]
    to: Format,

    #[clap(short, long, takes_value = false)]
    compact: bool,
}

#[derive(Debug, clap::ValueEnum, Clone)]
enum Format {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug)]
enum Value {
    Json(serde_json::Value),
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Value::Json(value) => value.serialize(serializer),
            Value::Toml(value) => value.serialize(serializer),
            Value::Yaml(value) => value.serialize(serializer),
        }
    }
}

fn read_input() -> String {
    let mut buf = String::new();
    let mut reader = BufReader::new(io::stdin());
    reader.read_to_string(&mut buf).expect("Can't read input");
    buf
}

fn write_output(output: &str) -> Result<()> {
    let mut writer = BufWriter::new(io::stdout());
    writer.write_all(output.as_bytes())?;
    Ok(())
}

fn load_input(input: &str, format: Format) -> Result<Value> {
    let value = match format {
        Format::Json => Value::Json(serde_json::from_str::<serde_json::Value>(input)?),
        Format::Yaml => Value::Yaml(serde_yaml::from_str::<serde_yaml::Value>(input)?),
        Format::Toml => Value::Toml(toml::from_str::<toml::Value>(input)?),
    };
    Ok(value)
}

fn dump_value(value: &Value, format: Format, is_compact: bool) -> Result<String> {
    let dumped: String = match (format, is_compact) {
        (Format::Json, true) => serde_json::to_string::<Value>(value)?,
        (Format::Json, false) => serde_json::to_string_pretty::<Value>(value)?,
        (Format::Yaml, _) => serde_yaml::to_string::<Value>(value)?,
        (Format::Toml, _) => toml::to_string::<Value>(value)?,
    };
    Ok(dumped)
}

fn main() {
    let args = CliArgs::parse();
    let input = read_input();
    let value = load_input(&input, args.from).expect("Can't deserialize input");
    let output = dump_value(&value, args.to, args.compact).expect("Can't serialize");
    write_output(&output).expect("Can't write output");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_json_to_yaml() {
        let input = r#"
{
    "array": ["a", "b", "c"],
    "the_answer": 42,
    "compact": false
}"#;

        let expected_output = r#"---
array:
  - a
  - b
  - c
compact: false
the_answer: 42
"#;

        let value = load_input(input, Format::Json).unwrap();
        let output = dump_value(&value, Format::Yaml, false).unwrap();

        assert_eq!(output, expected_output);
    }
}

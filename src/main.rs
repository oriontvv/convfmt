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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        Format::Json,
        r#"
{
    "array": ["a", "b"],
    "the_answer": 42,
    "compact": false
}"#,
        Format::Yaml,
        r#"---
array:
  - a
  - b
compact: false
the_answer: 42
"#,
        false
    )]
    #[case(
        Format::Json,
        r#"
{
    "array": ["a", "b"],
    "the_answer": 42,
    "compact": false
}"#,
        Format::Toml,
        r#"array = ["a", "b"]
compact = false
the_answer = 42
"#,
        false
    )]
    #[case(
        Format::Yaml,
        r#"---
array:
  - a
  - b
compact: false
the_answer: 42
"#,
        Format::Json,
        r#"{
  "array": [
    "a",
    "b"
  ],
  "compact": false,
  "the_answer": 42
}"#,
        false
    )]
    #[case(
        Format::Yaml,
        r#"---
array:
  - a
  - b
compact: false
the_answer: 42
"#,
        Format::Json,
        r#"{"array":["a","b"],"compact":false,"the_answer":42}"#,
        true
    )]
    #[case(
        Format::Yaml,
        r#"---
array:
  - a
  - b
compact: false
the_answer: 42
"#,
        Format::Toml,
        r#"array = ["a", "b"]
compact = false
the_answer = 42
"#,
        false
    )]
    #[case(
        Format::Toml,
        r#"array = ["a", "b"]
        compact = false
        the_answer = 42
        "#,
        Format::Yaml,
        r#"---
array:
  - a
  - b
compact: false
the_answer: 42
"#,
        false
    )]
    #[case(
        Format::Toml,
        r#"array = ["a", "b"]
        compact = false
        the_answer = 42
        "#,
        Format::Json,
        r#"{"array":["a","b"],"compact":false,"the_answer":42}"#,
        true
    )]
    fn test_convert_json_to_yaml(
        #[case] from_format: Format,
        #[case] input: &str,
        #[case] to_format: Format,
        #[case] expected_output: &str,
        #[case] is_compact: bool,
    ) {
        let value = load_input(input, from_format).unwrap();
        let output = dump_value(&value, to_format, is_compact).unwrap();

        assert_eq!(output, expected_output);
    }
}

use std::io::{self, Read, Write};

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

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
enum Format {
    Json,
    Yaml,
    Toml,
    Ron,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Value {
    Json(serde_json::Value),
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
    Ron(ron::Value),
}

fn read_input() -> Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    {
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buf)?;
    }
    Ok(buf)
}

fn write_output(output: &str) -> Result<()> {
    let stdout = io::stdout();
    {
        let mut handle = stdout.lock();
        handle.write_all(output.as_bytes())?
    }
    Ok(())
}

fn load_input(input: &str, format: Format) -> Result<Value> {
    let value = match format {
        Format::Json => Value::Json(serde_json::from_str::<serde_json::Value>(input)?),
        Format::Yaml => Value::Yaml(serde_yaml::from_str::<serde_yaml::Value>(input)?),
        Format::Toml => Value::Toml(toml::from_str::<toml::Value>(input)?),
        Format::Ron => Value::Ron(ron::from_str::<ron::Value>(input)?),
    };
    Ok(value)
}

fn dump_value(value: &Value, format: Format, is_compact: bool) -> Result<String> {
    let dumped: String = match (format, is_compact) {
        (Format::Json, true) => serde_json::to_string::<Value>(value)?,
        (Format::Json, false) => serde_json::to_string_pretty::<Value>(value)?,
        (Format::Yaml, _) => serde_yaml::to_string::<Value>(value)?,
        (Format::Toml, true) => toml::to_string::<Value>(value)?,
        (Format::Toml, false) => toml::to_string_pretty::<Value>(value)?,
        (Format::Ron, true) => ron::ser::to_string::<Value>(value)?,
        (Format::Ron, false) => {
            ron::ser::to_string_pretty::<Value>(value, ron::ser::PrettyConfig::default())?
        }
    };
    Ok(dumped)
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
        true
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
        r#"array = [
    'a',
    'b',
]
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
        true
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
    #[case(
        Format::Json,
        r#"{"array":["a","b"],"compact":false,"the_answer":42}"#,
        Format::Ron,
        r#"{"array":["a","b"],"compact":false,"the_answer":42}"#,
        true
    )]
    #[case(
        Format::Json,
        r#"{"array":["a","b"],"compact":false,"the_answer":42}"#,
        Format::Ron,
        r#"{
    "array": [
        "a",
        "b",
    ],
    "compact": false,
    "the_answer": 42,
}"#,
        false
    )]
    #[case(
        Format::Ron,
        r#"{"array":["a","b",],"compact":false,"the_answer":42,}"#,
        Format::Json,
        r#"{"array":["a","b"],"compact":false,"the_answer":42}"#,
        true
    )]
    fn test_convert_formats(
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

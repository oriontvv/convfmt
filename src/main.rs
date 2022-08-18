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

#[derive(Debug, Copy, Clone, PartialEq, clap::ValueEnum)]
enum Format {
    Json,
    Yaml,
    Toml,
    Ron,
    Json5,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum Value {
    Json(serde_json::Value),
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
    Ron(ron::Value),
    Json5(serde_json::Value),
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

fn load_input(input: &[u8], format: Format) -> Result<Value> {
    let value = match format {
        Format::Json => Value::Json(serde_json::from_slice::<serde_json::Value>(input)?),
        Format::Yaml => Value::Yaml(serde_yaml::from_slice::<serde_yaml::Value>(input)?),
        Format::Toml => Value::Toml(toml::from_slice::<toml::Value>(input)?),
        Format::Ron => Value::Ron(ron::de::from_bytes::<ron::Value>(input)?),
        Format::Json5 => {
            let s = std::str::from_utf8(input)?;
            Value::Json5(serde_json::from_str::<serde_json::Value>(s)?)
        }
    };
    Ok(value)
}

fn dump_value(value: &Value, format: Format, is_compact: bool) -> Result<Vec<u8>> {
    let dumped: Vec<u8> = match (format, is_compact) {
        (Format::Json, true) => serde_json::to_vec::<Value>(value)?,
        (Format::Json, false) => serde_json::to_vec_pretty::<Value>(value)?,
        (Format::Yaml, _) => serde_yaml::to_vec::<Value>(value)?,
        (Format::Toml, true) => toml::to_vec::<Value>(value)?,
        (Format::Toml, false) => toml::to_string_pretty::<Value>(value).map(|e| e.into_bytes())?,
        (Format::Ron, true) => ron::ser::to_string::<Value>(value).map(|e| e.into_bytes())?,
        (Format::Ron, false) => {
            ron::ser::to_string_pretty::<Value>(value, ron::ser::PrettyConfig::default())
                .map(|e| e.into_bytes())?
        }
        (Format::Json5, _) => json5::to_string::<Value>(value).map(|e| e.into_bytes())?,
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

    fn get_test_value(format: Format, is_compact: bool) -> String {
        match (format, is_compact) {
            (Format::Json, true) => {
                r#"{"array":["a","b"],"boolean":false,"the_answer":42}"#.to_string()
            }
            (Format::Json, false) => r#"{
  "array": [
    "a",
    "b"
  ],
  "boolean": false,
  "the_answer": 42
}"#
            .to_string(),
            (Format::Yaml, _) => r#"---
array:
  - a
  - b
boolean: false
the_answer: 42
"#
            .to_string(),
            (Format::Toml, true) => r#"array = ["a", "b"]
boolean = false
the_answer = 42
"#
            .to_string(),
            (Format::Toml, false) => r#"array = [
    'a',
    'b',
]
boolean = false
the_answer = 42
"#
            .to_string(),
            (Format::Ron, true) => {
                r#"{"array":["a","b"],"boolean":false,"the_answer":42}"#.to_string()
            }
            (Format::Ron, false) => r#"{
    "array": [
        "a",
        "b",
    ],
    "boolean": false,
    "the_answer": 42,
}"#
            .to_string(),
            (Format::Json5, _) => {
                r#"{"array":["a","b"],"boolean":false,"the_answer":42}"#.to_string()
            }
        }
    }

    #[rstest]
    #[case(Format::Json, Format::Yaml, false)]
    #[case(Format::Json, Format::Toml, true)]
    #[case(Format::Json, Format::Toml, false)]
    #[case(Format::Yaml, Format::Json, false)]
    #[case(Format::Yaml, Format::Json, true)]
    #[case(Format::Yaml, Format::Toml, true)]
    #[case(Format::Toml, Format::Yaml, false)]
    #[case(Format::Toml, Format::Json, true)]
    #[case(Format::Json, Format::Ron, true)]
    #[case(Format::Json, Format::Ron, false)]
    #[case(Format::Ron, Format::Json, true)]
    #[case(Format::Json5, Format::Json, true)]
    #[case(Format::Json, Format::Json5, true)]
    #[case(Format::Json, Format::Json5, false)]
    #[case(Format::Json5, Format::Json, false)]
    fn test_convert_formats(
        #[case] from_format: Format,
        #[case] to_format: Format,
        #[case] is_compact: bool,
    ) {
        let input = get_test_value(from_format, is_compact);
        let expected_output = get_test_value(to_format, is_compact);

        let value = load_input(input.as_bytes(), from_format).unwrap();
        let output = String::from_utf8(dump_value(&value, to_format, is_compact).unwrap()).unwrap();

        assert_eq!(output, expected_output);
    }
}

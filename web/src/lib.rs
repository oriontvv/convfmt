//! Wasm bindings for [`convfmt`], used by the web version.
//!
//! The whole conversion happens in the browser, there is no backend involved.
//! Data is passed as bytes in both directions, so binary formats (e.g. bson)
//! work the same way as textual ones.

use anyhow::{Result, anyhow};
use clap::ValueEnum;
use convfmt::{Format, dump_value, load_input};
use wasm_bindgen::prelude::*;

/// Version of the underlying `convfmt` crate.
#[wasm_bindgen]
pub fn version() -> String {
    convfmt::VERSION.to_owned()
}

/// Names of all supported formats, as accepted by the cli `--from`/`--to`.
#[wasm_bindgen]
pub fn formats() -> Vec<String> {
    Format::value_variants()
        .iter()
        .filter_map(|format| format.to_possible_value())
        .map(|value| value.get_name().to_owned())
        .collect()
}

/// Whether the format can't be displayed as text and needs a file instead.
#[wasm_bindgen]
pub fn is_binary(format: &str) -> bool {
    parse_format(format).is_ok_and(Format::is_binary)
}

#[wasm_bindgen]
pub fn convert(
    input: &[u8],
    from: &str,
    to: &str,
    compact: bool,
    sort_keys: bool,
) -> Result<Vec<u8>, JsError> {
    convert_named(input, from, to, compact, sort_keys)
        .map_err(|err| JsError::new(&format!("{err:#}")))
}

fn convert_named(
    input: &[u8],
    from: &str,
    to: &str,
    compact: bool,
    sort_keys: bool,
) -> Result<Vec<u8>> {
    let from = parse_format(from)?;
    let to = parse_format(to)?;
    convert_bytes(input, from, to, compact, sort_keys)
}

/// The same pipeline as the cli one in `src/main.rs`.
fn convert_bytes(
    input: &[u8],
    from: Format,
    to: Format,
    compact: bool,
    sort_keys: bool,
) -> Result<Vec<u8>> {
    let mut value = load_input(input, from)?;
    if sort_keys {
        convfmt::sort_keys(&mut value);
    }
    dump_value(&value, to, compact)
}

fn parse_format(format: &str) -> Result<Format> {
    <Format as ValueEnum>::from_str(format, true).map_err(|err| anyhow!("{err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_str(input: &str, from: Format, to: Format, compact: bool, sort: bool) -> String {
        let output = convert_bytes(input.as_bytes(), from, to, compact, sort).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn converts_yaml_to_json() {
        let output = convert_str("the_answer: 42", Format::Yaml, Format::Json, true, false);
        assert_eq!(output, r#"{"the_answer":42}"#);
    }

    #[test]
    fn preserves_key_order_by_default() {
        let output = convert_str(
            r#"{"b": 1, "a": 2}"#,
            Format::Json,
            Format::Json,
            true,
            false,
        );
        assert_eq!(output, r#"{"b":1,"a":2}"#);
    }

    #[test]
    fn sorts_keys_on_demand() {
        let output = convert_str(
            r#"{"b": 1, "a": 2}"#,
            Format::Json,
            Format::Json,
            true,
            true,
        );
        assert_eq!(output, r#"{"a":2,"b":1}"#);
    }

    #[test]
    fn pretty_output_is_the_default() {
        let output = convert_str(r#"{"a":1}"#, Format::Json, Format::Json, false, false);
        assert!(
            output.contains('\n'),
            "expected pretty output, got {output}"
        );
    }

    #[test]
    fn reports_broken_input() {
        let err = convert_bytes(b"{not json", Format::Json, Format::Yaml, false, false)
            .expect_err("broken input should fail");
        assert!(!format!("{err:#}").is_empty());
    }

    #[test]
    fn parses_format_names_case_insensitively() {
        assert_eq!(parse_format("YAML").unwrap(), Format::Yaml);
        assert!(parse_format("nope").is_err());
    }

    #[test]
    fn exposes_formats_and_version() {
        let formats = formats();
        assert!(formats.contains(&"json".to_owned()));
        assert!(formats.contains(&"hocon".to_owned()));
        assert_eq!(version(), convfmt::VERSION);
    }

    #[test]
    fn only_bson_is_binary() {
        assert!(is_binary("bson"));
        assert!(!is_binary("json"));
        assert!(!is_binary("nope"));
    }
}

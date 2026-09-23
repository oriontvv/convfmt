mod csv_value;
#[cfg(feature = "hocon")]
mod hocon_value;
mod jsonl_value;
mod sort;
mod xml_value;

use anyhow::Result;
use serde::Serialize;

#[cfg(feature = "hocon")]
use crate::hocon_value::{HoconWrapper, load_hocon};
use crate::{
    csv_value::{CsvWrapper, json_to_csv, load_csv},
    jsonl_value::{JsonlWrapper, json_to_jsonl, load_jsonl},
    xml_value::{XmlWrapper, json_to_xml, load_xml},
};

pub use crate::sort::sort_keys;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Copy, Clone, PartialEq, clap::ValueEnum)]
pub enum Format {
    Bson,
    Csv,
    Hjson,
    #[cfg(feature = "hocon")]
    Hocon,
    Json,
    Json5,
    Jsonl,
    Plist,
    Ron,
    Toml,
    Toon,
    Xml,
    Yaml,
}

impl Format {
    pub const fn is_binary(self) -> bool {
        matches!(self, Format::Bson)
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
    Bson(bson::Bson),
    Csv(CsvWrapper),
    Hjson(serde_hjson::Value),
    #[cfg(feature = "hocon")]
    Hocon(HoconWrapper),
    Json(serde_json::Value),
    Json5(serde_json::Value),
    Jsonl(JsonlWrapper),
    Plist(plist::Value),
    Ron(ron::Value),
    Toml(toml::Value),
    Toon(serde_json::Value),
    Xml(XmlWrapper),
    Yaml(serde_yaml_neo::Value),
}

pub fn load_input(input: &[u8], format: Format) -> Result<Value> {
    let value = match format {
        Format::Bson => Value::Bson(bson::deserialize_from_slice(input)?),
        Format::Csv => Value::Csv(load_csv(input)?),
        Format::Hjson => Value::Hjson(serde_hjson::from_slice(input)?),
        #[cfg(feature = "hocon")]
        Format::Hocon => Value::Hocon(load_hocon(input)?),
        Format::Json => Value::Json(serde_json::from_slice(input)?),
        Format::Json5 => Value::Json5(json5::from_str(str::from_utf8(input)?)?),
        Format::Jsonl => Value::Jsonl(load_jsonl(input)?),
        Format::Plist => Value::Plist(plist::from_bytes(input)?),
        Format::Ron => Value::Ron(ron::de::from_bytes(input)?),
        Format::Toml => {
            let s = std::str::from_utf8(input)?;
            Value::Toml(toml::from_str(s)?)
        }
        Format::Toon => {
            let s = std::str::from_utf8(input)?;
            Value::Toon(toon_format::decode_default(s)?)
        }
        Format::Xml => Value::Xml(load_xml(input)?),
        Format::Yaml => Value::Yaml(serde_yaml_neo::from_slice(input)?),
    };
    Ok(value)
}

pub fn dump_value(value: &Value, format: Format, is_compact: bool) -> Result<Vec<u8>> {
    let dumped: Vec<u8> = match (format, is_compact) {
        (Format::Bson, _) => bson::serialize_to_vec(value)?,
        (Format::Csv, _) => {
            let json_dumped = serde_json::to_vec(value)?;
            json_to_csv(&json_dumped)?
        }
        (Format::Hjson, _) => serde_hjson::to_vec(value)?,
        #[cfg(feature = "hocon")]
        (Format::Hocon, true) => serde_json::to_vec(value)?,
        #[cfg(feature = "hocon")]
        (Format::Hocon, false) => serde_json::to_vec_pretty(value)?,
        (Format::Json, true) => serde_json::to_vec(value)?,
        (Format::Json, false) => serde_json::to_vec_pretty(value)?,
        (Format::Json5, _) => json5::to_string(value).map(|e| e.into_bytes())?,
        (Format::Jsonl, _) => {
            let json_dumped = serde_json::to_vec(value)?;
            json_to_jsonl(&json_dumped)?
        }
        (Format::Plist, _) => {
            let mut buffer = Vec::new();
            plist::to_writer_xml(&mut buffer, value)?;
            buffer
        }
        (Format::Ron, true) => ron::ser::to_string(value).map(|e| e.into_bytes())?,
        (Format::Ron, false) => ron::ser::to_string_pretty(
            value,
            ron::ser::PrettyConfig::default().new_line("\n".to_owned()),
        )
        .map(|e| e.into_bytes())?,
        (Format::Toml, true) => toml::to_string(value).map(|e| e.into_bytes())?,
        (Format::Toml, false) => toml::to_string_pretty(value).map(|e| e.into_bytes())?,
        (Format::Toon, _) => toon_format::encode_default(value)?.as_bytes().to_vec(),
        (Format::Xml, _) => {
            let json_dumped = serde_json::to_vec(value)?;
            json_to_xml(&json_dumped)?
        }
        (Format::Yaml, true) => serde_yaml_neo::to_string(value).map(|e| e.into_bytes())?,
        (Format::Yaml, false) => {
            let yaml = serde_yaml_neo::to_string(value)?;
            indent_yaml_block_sequences(&yaml).into_bytes()
        }
    };
    Ok(dumped)
}

/// `serde_yaml_neo` (like the underlying libyaml) always emits block sequences
/// that are mapping values flush with their key, e.g. `key:\n- item`. This adds
/// the conventional extra 2-space indent (`key:\n  - item`) for the pretty
/// (non-compact) output, propagating it to everything nested under such a
/// sequence.
fn indent_yaml_block_sequences(yaml: &str) -> String {
    let had_trailing_newline = yaml.ends_with('\n');
    let mut raw_lines: Vec<&str> = yaml.split('\n').collect();
    if had_trailing_newline {
        raw_lines.pop();
    }

    let mut stack: Vec<usize> = Vec::new();
    let mut prev: Option<(usize, String)> = None;
    let mut out_lines: Vec<String> = Vec::with_capacity(raw_lines.len());

    for line in raw_lines {
        let indent = line.len() - line.trim_start_matches(' ').len();
        let content = &line[indent..];

        if content.is_empty() {
            out_lines.push(String::new());
            continue;
        }

        let is_seq_item = content == "-" || content.starts_with("- ");

        while let Some(&top) = stack.last() {
            let still_active = indent > top || (indent == top && is_seq_item);
            if still_active {
                break;
            }
            stack.pop();
        }

        if is_seq_item {
            if let Some((prev_indent, ref prev_content)) = prev {
                let prev_is_seq_item = prev_content == "-" || prev_content.starts_with("- ");
                if prev_indent == indent && !prev_is_seq_item && prev_content.ends_with(':') {
                    stack.push(indent);
                }
            }
        }

        let shift = stack.len() * 2;
        out_lines.push(format!("{}{}", " ".repeat(indent + shift), content));

        prev = Some((indent, content.to_string()));
    }

    let mut result = out_lines.join("\n");
    if had_trailing_newline {
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests;

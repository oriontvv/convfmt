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
            serde_yaml_neo::to_string_with_indent(value, 2).map(|e| e.into_bytes())?
        }
    };
    Ok(dumped)
}

#[cfg(test)]
mod tests;

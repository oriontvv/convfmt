mod hocon_value;
mod xml_value;

use anyhow::Result;
use serde::Serialize;

use crate::{
    hocon_value::{load_hocon, HoconWrapper},
    xml_value::{load_xml, XmlWrapper},
};

#[derive(Debug, Copy, Clone, PartialEq, clap::ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
    Ron,
    Json5,
    Bson,
    Hocon,
    Xml,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
    Json(serde_json::Value),
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
    Ron(ron::Value),
    Json5(serde_json::Value),
    Bson(bson::Bson),
    Hocon(HoconWrapper),
    Xml(XmlWrapper),
}

pub fn load_input(input: &[u8], format: Format) -> Result<Value> {
    let value = match format {
        Format::Json => Value::Json(serde_json::from_slice(input)?),
        Format::Yaml => Value::Yaml(serde_yaml::from_slice(input)?),
        Format::Toml => {
            let s = std::str::from_utf8(input)?;
            Value::Toml(toml::from_str(s)?)
        }
        Format::Ron => Value::Ron(ron::de::from_bytes(input)?),
        Format::Json5 => Value::Json5(serde_json::from_slice(input)?),
        Format::Bson => Value::Bson(bson::from_slice(input)?),
        Format::Hocon => Value::Hocon(load_hocon(input)?),
        Format::Xml => Value::Xml(load_xml(input)?),
    };
    Ok(value)
}

pub fn dump_value(value: &Value, format: Format, is_compact: bool) -> Result<Vec<u8>> {
    let dumped: Vec<u8> = match (format, is_compact) {
        (Format::Json, true) => serde_json::to_vec(value)?,
        (Format::Json, false) => serde_json::to_vec_pretty(value)?,
        (Format::Yaml, _) => serde_yaml::to_string(value).map(|e| e.into_bytes())?,
        (Format::Toml, true) => toml::to_string(value).map(|e| e.into_bytes())?,
        (Format::Toml, false) => toml::to_string_pretty(value).map(|e| e.into_bytes())?,
        (Format::Ron, true) => ron::ser::to_string(value).map(|e| e.into_bytes())?,
        (Format::Ron, false) => ron::ser::to_string_pretty(
            value,
            ron::ser::PrettyConfig::default().new_line("\n".to_owned()),
        )
        .map(|e| e.into_bytes())?,
        (Format::Json5, _) => json5::to_string(value).map(|e| e.into_bytes())?,
        (Format::Bson, _) => bson::to_vec(value)?,
        (Format::Hocon, _) => unimplemented!("Sorry, hocon output format is not implemented yet"),
        (Format::Xml, _) => todo!(),
    };
    Ok(dumped)
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
            (Format::Yaml, _) => r#"array:
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
    "a",
    "b",
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
            (Format::Bson, _) => {
                        "A\0\0\0\u{4}array\0\u{17}\0\0\0\u{2}0\0\u{2}\0\0\0a\0\u{2}1\0\u{2}\0\0\0b\0\0\u{8}boolean\0\0\u{12}the_answer\0*\0\0\0\0\0\0\0\0".to_string()
                    }
            (Format::Hocon, _) => r#"
array: [a,b]
boolean: false
the_answer: 42
"#
                    .to_string(),
            (Format::Xml, _) => {
                        r#"
<?xml version="1.0" encoding="UTF-8" ?>
<array>a</array>
<array>b</array>
<boolean>false</boolean>
<the_answer>42</the_answer>
                        "#.to_string()
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
    #[case(Format::Json, Format::Bson, false)]
    #[case(Format::Bson, Format::Json5, true)]
    #[case(Format::Hocon, Format::Json, false)]
    #[case(Format::Xml, Format::Yaml, false)]
    #[case(Format::Toml, Format::Xml, true)]
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

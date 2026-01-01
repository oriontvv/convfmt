mod csv_value;
mod hocon_value;
mod jsonl_value;
mod xml_value;

use anyhow::Result;
use serde::Serialize;

use crate::{
    csv_value::{CsvWrapper, json_to_csv, load_csv},
    hocon_value::{HoconWrapper, load_hocon},
    jsonl_value::{JsonlWrapper, json_to_jsonl, load_jsonl},
    xml_value::{XmlWrapper, json_to_xml, load_xml},
};

#[derive(Debug, Copy, Clone, PartialEq, clap::ValueEnum)]
pub enum Format {
    Bson,
    Csv,
    Hjson,
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

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
    Bson(bson::Bson),
    Csv(CsvWrapper),
    Hjson(serde_hjson::Value),
    Hocon(HoconWrapper),
    Json(serde_json::Value),
    Json5(serde_json::Value),
    Jsonl(JsonlWrapper),
    Plist(plist::Value),
    Ron(ron::Value),
    Toml(toml::Value),
    Toon(serde_json::Value),
    Xml(XmlWrapper),
    Yaml(serde_yaml::Value),
}

pub fn load_input(input: &[u8], format: Format) -> Result<Value> {
    let value = match format {
        Format::Bson => Value::Bson(bson::deserialize_from_slice(input)?),
        Format::Csv => Value::Csv(load_csv(input)?),
        Format::Hjson => Value::Hjson(serde_hjson::from_slice(input)?),
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
        Format::Yaml => Value::Yaml(serde_yaml::from_slice(input)?),
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
        (Format::Hocon, true) => serde_json::to_vec(value)?,
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
        (Format::Yaml, _) => serde_yaml::to_string(value).map(|e| e.into_bytes())?,
    };
    Ok(dumped)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn get_test_value(format: Format, is_compact: bool) -> String {
        let value = match (format, is_compact) {
            (Format::Bson, _) => {
                "A\0\0\0\u{4}array\0\u{17}\0\0\0\u{2}0\0\u{2}\0\0\0a\0\u{2}1\0\u{2}\0\0\0b\0\0\u{8}boolean\0\0\u{12}the_answer\0*\0\0\0\0\0\0\0\0"
            }
            (Format::Csv, _) => unimplemented!("use raw data for tests"),
            (Format::Hjson, _) => {
                r#"{
  array:
  [
    a
    b
  ]
  boolean: false
  the_answer: 42
}"#
            }
            (Format::Hocon, _) => {
                r#"
array: [a,b]
boolean: false
the_answer: 42
"#
            }
            (Format::Json, true) => r#"{"array":["a","b"],"boolean":false,"the_answer":42}"#,
            (Format::Json, false) => {
                r#"{
  "array": [
    "a",
    "b"
  ],
  "boolean": false,
  "the_answer": 42
}"#
            }
            (Format::Json5, _) => {
                r#"{
  array: [
    "a",
    "b",
  ],
  boolean: false,
  the_answer: 42,
}"#
            }
            (Format::Jsonl, _) => unimplemented!("use raw data for tests"),
            (Format::Plist, _) => {
                r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>array</key>
	<array>
		<string>a</string>
		<string>b</string>
	</array>
	<key>boolean</key>
	<false/>
	<key>the_answer</key>
	<integer>42</integer>
</dict>
</plist>"#
            }
            (Format::Ron, true) => r#"{"array":["a","b"],"boolean":false,"the_answer":42}"#,
            (Format::Ron, false) => {
                r#"{
    "array": [
        "a",
        "b",
    ],
    "boolean": false,
    "the_answer": 42,
}"#
            }
            (Format::Toml, true) => {
                r#"array = ["a", "b"]
boolean = false
the_answer = 42
"#
            }
            (Format::Toml, false) => {
                r#"array = [
    "a",
    "b",
]
boolean = false
the_answer = 42
"#
            }
            (Format::Toon, _) => {
                r#"array[2]: a,b
boolean: false
the_answer: 42"#
            }
            (Format::Xml, _) => {
                r#"<root><array>a</array><array>b</array><boolean>false</boolean><the_answer>42</the_answer></root>"#
            }
            (Format::Yaml, _) => {
                r#"array:
- a
- b
boolean: false
the_answer: 42
"#
            }
        };
        value.to_string()
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
    #[case(Format::Toml, Format::Hjson, true)]
    #[case(Format::Hjson, Format::Json, false)]
    #[case(Format::Toon, Format::Yaml, false)]
    #[case(Format::Toon, Format::Json, true)]
    #[case(Format::Yaml, Format::Toon, false)]
    #[case(Format::Yaml, Format::Toon, true)]
    #[case(Format::Json, Format::Plist, true)]
    #[case(Format::Plist, Format::Yaml, true)]
    fn test_convert_formats(
        #[case] from_format: Format,
        #[case] to_format: Format,
        #[case] is_compact: bool,
    ) {
        println!("{from_format:?} -> {to_format:?}. is_compact: {is_compact}");

        let input = get_test_value(from_format, is_compact);
        let expected_output = get_test_value(to_format, is_compact);

        let value = load_input(input.as_bytes(), from_format).unwrap();
        let output = String::from_utf8(dump_value(&value, to_format, is_compact).unwrap()).unwrap();

        assert_eq!(output, expected_output);
    }

    #[rstest]
    #[case(
        Format::Csv,
        Format::Json,
        r#"age,immortal,name,power
55000,true,Gendalf,50.0
50,false,Frodo,5.0
"#,
        r#"[
  {
    "age": 55000,
    "immortal": true,
    "name": "Gendalf",
    "power": 50.0
  },
  {
    "age": 50,
    "immortal": false,
    "name": "Frodo",
    "power": 5.0
  }
]"#,
        false
    )]
    #[case(
        Format::Csv,
        Format::Json,
        r#"age,immortal,name,power,test_empty
55000, true, Gendalf, 50.0,
50, false, Frodo, 5.0,
"#,
        r#"[{"age":55000,"immortal":true,"name":"Gendalf","power":50.0,"test_empty":null},{"age":50,"immortal":false,"name":"Frodo","power":5.0,"test_empty":null}]"#,
        true
    )]
    #[case(
        Format::Json,
        Format::Csv,
        r#"[{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0},{"age":50,"immortal":false,"name":"Frodo","power":5.0}]"#,
        r#"age,immortal,name,power
55000,true,"Gendalf the \"White\"",50.0
50,false,"Frodo",5.0
"#,
        true
    )]
    #[case(
        Format::Hocon,
        Format::Json,
        r#"{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0}"#,
        r#"{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0}"#,
        true
    )]
    #[case(
        Format::Json,
        Format::Json,
        r#"[{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0},{"age":50,"immortal":false,"name":"Frodo","power":5.0}]"#,
        r#"[{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0},{"age":50,"immortal":false,"name":"Frodo","power":5.0}]"#,
        true
    )]
    #[case(
        Format::Jsonl,
        Format::Xml,
        r#"{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0}
{"age":50,"immortal":false,"name":"Frodo","power":5.0}
"#,
    r#"<root><age>55000</age><immortal>true</immortal><name>Gendalf the &quot;White&quot;</name><power>50.0</power></root>
<root><age>50</age><immortal>false</immortal><name>Frodo</name><power>5.0</power></root>
"#,
        false
    )]
    #[case(
    Format::Json,
    Format::Jsonl,
    r#"[{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0},{"age":50,"immortal":false,"name":"Frodo","power":5.0}]"#,
    r#"{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0}
{"age":50,"immortal":false,"name":"Frodo","power":5.0}
"#,
    false
)]
    fn test_raw_convert(
        #[case] from_format: Format,
        #[case] to_format: Format,
        #[case] input: &str,
        #[case] expected_output: &str,
        #[case] is_compact: bool,
    ) {
        let value = load_input(input.as_bytes(), from_format).unwrap();
        let output = String::from_utf8(dump_value(&value, to_format, is_compact).unwrap()).unwrap();

        assert_eq!(output, expected_output);
    }
}

use std::iter::zip;

use anyhow::{Result, bail};
use java_properties::{PropertiesIter, read};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize)]
pub struct JavaPropertiesWrapper {
    pub items: serde_json::Value,
}

impl Serialize for JavaPropertiesWrapper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.items.serialize(serializer)
    }
}

pub fn load_javaproperties(bytes: &[u8]) -> Result<JavaPropertiesWrapper> {
    // read(input)cal

    PropertiesIter::new(BufReader::new(f)).read_into(|k, v| {
        //    dst_map2.insert(k, v);
    })?;

    /*
    let mut reader = csv::Reader::from_reader(bytes);

    let header: Vec<String> = match reader.headers() {
        Ok(headers) => headers.iter().map(|s| s.to_string()).collect(),
        Err(e) => return Result::Err(e.into()),
    };
    let mut items = Vec::new();
    for record_result in reader.records() {
        let mut record_map = serde_json::Map::new();
        for (key, value) in zip(header.clone(), &record_result?) {
            record_map.insert(key, parse_value(value.trim()));
        }
        items.push(serde_json::Value::Object(record_map));
    }
    */
    Ok(JavaPropertiesWrapper {
        items: serde_json::Value::Object(items),
    })
}

fn parse_value(s: &str) -> JsonValue {
    if s.is_empty() {
        return JsonValue::Null;
    }
    if let Ok(b) = s.parse::<bool>() {
        return JsonValue::Bool(b);
    }
    if let Ok(i) = s.parse::<i64>() {
        return JsonValue::Number(i.into());
    }
    if let Ok(f) = s.parse::<f64>()
        && f.is_finite()
    {
        return JsonValue::Number(serde_json::Number::from_f64(f).unwrap());
    }
    JsonValue::String(s.to_string())
}

pub fn json_to_javaproperties(json: &[u8]) -> Result<Vec<u8>> {
    let json: JsonValue = serde_json::from_slice(json)?;

    if let JsonValue::Array(values) = json {
        let mut buffer = String::new();
        let mut dump_header: bool = true;

        for item in values {
            if let JsonValue::Object(items) = item {
                let keys: Vec<String> = items.keys().cloned().collect();
                if dump_header {
                    dump_header = false;
                    let mut first = true;
                    for key in keys.clone() {
                        if first {
                            first = false;
                        } else {
                            buffer.push(',');
                        }
                        buffer.push_str(&key.to_string());
                    }
                    buffer.push('\n');
                }
                let mut first = true;
                for key in keys {
                    if let Some(value) = items.get(&key) {
                        if first {
                            first = false;
                        } else {
                            buffer.push(',');
                        }
                        buffer.push_str(&value.to_string());
                    } else {
                        bail!("Missing value for key: in {key:?}")
                    }
                }
            } else {
                bail!("Invalid json format for csv convertation: {item:?}")
            }
            buffer.push('\n');
        }
        Ok(buffer.into_bytes())
    } else {
        bail!("Invalid json format for csv convertation. Expected root Array of items.")
    }
}

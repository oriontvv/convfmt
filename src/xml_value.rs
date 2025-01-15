use anyhow::{bail, Context, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value as JsonValue};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct XmlWrapper(serde_json::Value);

pub fn load_xml(xml_str: &[u8]) -> Result<XmlWrapper> {
    let mut reader = Reader::from_reader(xml_str);

    let mut stack = Vec::new();
    let mut current_map = Map::new();
    let mut current_name = String::new();
    let mut buffer = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name).into_owned();
                let mut attributes = Map::new();

                for attr in e.attributes() {
                    let attr = attr?;
                    let key = attr.key.as_ref().to_vec();
                    let key = String::from_utf8_lossy(&key).into_owned();
                    let value = attr.value.as_ref().to_vec();
                    let value = String::from_utf8_lossy(&value).into_owned();
                    attributes.insert(format!("@{}", key), parse_value(&value));
                }

                if !attributes.is_empty() {
                    current_map.insert(name.clone(), JsonValue::Object(attributes));
                }

                stack.push((current_map, current_name));
                current_map = Map::new();
                current_name = name;
            }
            Ok(Event::Text(e)) => {
                let text = e
                    .unescape()
                    .context(format!("XML unescape error: {:?}", e))?;
                buffer = text.into_owned();
            }
            Ok(Event::CData(e)) => {
                buffer = String::from_utf8_lossy(e.as_ref()).into_owned();
            }
            Ok(Event::End(_)) => {
                let value = if current_map.is_empty() {
                    parse_value(&buffer.trim())
                } else {
                    if !buffer.trim().is_empty() {
                        current_map.insert(
                            "#text".to_string(),
                            JsonValue::String(buffer.trim().to_string()),
                        );
                    }
                    JsonValue::Object(current_map)
                };

                let (mut parent_map, parent_name) = stack.pop().unwrap();

                if parent_map.contains_key(&current_name) {
                    let existing_value = parent_map.get_mut(&current_name).unwrap();
                    if let JsonValue::Array(ref mut arr) = existing_value {
                        arr.push(value);
                    } else {
                        let old_value = parent_map.remove(&current_name).unwrap();
                        parent_map.insert(
                            current_name.clone(),
                            JsonValue::Array(vec![old_value, value]),
                        );
                    }
                } else {
                    parent_map.insert(current_name.clone(), value);
                }

                current_map = parent_map;
                current_name = parent_name;
                buffer.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => (),
        }
    }

    if stack.is_empty() && !current_map.is_empty() {
        Ok(XmlWrapper(JsonValue::Object(current_map)))
    } else {
        bail!("Can't read xml");
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
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
    if let Ok(f) = s.parse::<f64>() {
        if f.is_finite() {
            return JsonValue::Number(serde_json::Number::from_f64(f).unwrap());
        }
    }
    JsonValue::String(s.to_string())
}

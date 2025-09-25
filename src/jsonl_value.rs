use anyhow::{Result, bail};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize)]
pub struct JsonlWrapper {
    pub items: Vec<JsonValue>,
}

impl Serialize for JsonlWrapper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.items.len()))?;
        for item in &self.items {
            seq.serialize_element(&item)?;
            seq.serialize_element(&'\n')?;
        }
        seq.end()
    }
}

pub fn load_jsonl(bytes: &[u8]) -> Result<JsonlWrapper> {
    let mut items = vec![];

    let lines = std::str::from_utf8(bytes)?.lines();
    for line in lines {
        let item = serde_json::from_str(line)?;
        items.push(item);
    }

    Ok(JsonlWrapper { items })
}

pub fn json_to_jsonl(json: &[u8]) -> Result<Vec<u8>> {
    let json: JsonValue = serde_json::from_slice(json)?;
    if let JsonValue::Array(items) = json {
        let mut buffer = String::new();
        for item in items {
            let s = serde_json::to_string(&item)?;
            buffer.push_str(&s);
            buffer.push('\n');
        }
        Ok(buffer.into_bytes())
    } else {
        bail!("Invalid json format for jsonl convertation. Expected root Array of items.")
    }
}

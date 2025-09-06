use anyhow::Result;
use hocon::{Hocon, HoconLoader};
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HoconWrapper(serde_json::Value);

pub fn load_hocon(input: &[u8]) -> Result<HoconWrapper> {
    let s = std::str::from_utf8(input)?;
    let hocon = HoconLoader::new().load_str(s)?.hocon()?;
    let json = hocon_to_json(hocon)?;
    Ok(HoconWrapper(json))
}

fn hocon_to_json(hocon: Hocon) -> Result<serde_json::Value> {
    match hocon {
        Hocon::Boolean(b) => Ok(serde_json::Value::Bool(b)),
        Hocon::Integer(i) => Ok(serde_json::Value::Number(serde_json::Number::from(i))),
        Hocon::Real(f) => Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(f).unwrap(),
        )),
        Hocon::String(s) => Ok(serde_json::Value::String(s)),
        Hocon::Array(vec) => {
            let json_array: Result<Vec<serde_json::Value>> =
                vec.into_iter().map(hocon_to_json).collect();
            Ok(serde_json::Value::Array(json_array?))
        }
        Hocon::Hash(map) => {
            let json_object: Result<serde_json::Map<String, serde_json::Value>> = map
                .into_iter()
                .map(|(k, v)| Ok((k, hocon_to_json(v)?)))
                .collect();

            Ok(serde_json::Value::Object(json_object?))
        }
        Hocon::Null => Ok(serde_json::Value::Null),
        Hocon::BadValue(bad_value) => Err(anyhow::Error::from(bad_value)),
    }
}

pub fn json_to_hocon(json: &[u8]) -> Result<Vec<u8>> {
    let value: serde_json::Value = serde_json::from_slice(json)?;
    let _hocon = dump_hocon(value)?;
    // Ok(serde_hocon::to_vec(&_hocon))
    todo!("missing hocon serialization support")
}

fn dump_hocon(json: serde_json::Value) -> Result<hocon::Hocon> {
    match json {
        serde_json::Value::Null => Ok(Hocon::Null),
        serde_json::Value::Bool(b) => Ok(Hocon::Boolean(b)),
        serde_json::Value::Number(number) => {
            if number.is_f64() {
                let value = number.as_f64().expect("Can't convert hocon f64");
                Ok(Hocon::Real(value))
            } else {
                let value = number.as_i64().expect("Can't convert hocon i64");
                Ok(Hocon::Integer(value))
            }
        }
        serde_json::Value::String(s) => Ok(Hocon::String(s)),
        serde_json::Value::Array(vec) => {
            let hocon_array: Result<Vec<hocon::Hocon>> = vec.into_iter().map(dump_hocon).collect();
            Ok(Hocon::Array(hocon_array?))
        }
        serde_json::Value::Object(map) => {
            let mut hm = LinkedHashMap::new();

            for (k, v) in map.into_iter() {
                let value = dump_hocon(v)?;
                hm.insert(k, value);
            }
            Ok(Hocon::Hash(hm))
        }
    }
}

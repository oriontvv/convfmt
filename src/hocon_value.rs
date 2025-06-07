use anyhow::Result;
use hocon::{Hocon, HoconLoader};
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

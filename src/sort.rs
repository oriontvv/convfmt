use std::cmp::Ordering;
use std::mem::take;

use crate::Value;

/// Recursively sorts keys of every object/map inside the value.
pub fn sort_keys(value: &mut Value) {
    match value {
        Value::Bson(v) => sort_bson(v),
        Value::Csv(v) => sort_json(&mut v.items),
        Value::Hjson(v) => sort_hjson(v),
        #[cfg(feature = "hocon")]
        Value::Hocon(v) => sort_json(&mut v.0),
        Value::Json(v) | Value::Json5(v) | Value::Toon(v) => sort_json(v),
        Value::Jsonl(v) => v.items.iter_mut().for_each(sort_json),
        Value::Plist(v) => sort_plist(v),
        Value::Ron(v) => sort_ron(v),
        Value::Toml(v) => sort_toml(v),
        Value::Xml(v) => sort_json(&mut v.0),
        Value::Yaml(v) => sort_yaml(v),
    }
}

fn sort_bson(value: &mut bson::Bson) {
    match value {
        bson::Bson::Document(doc) => {
            let mut entries: Vec<(String, bson::Bson)> = take(doc).into_iter().collect();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            *doc = entries
                .into_iter()
                .map(|(key, mut value)| {
                    sort_bson(&mut value);
                    (key, value)
                })
                .collect();
        }
        bson::Bson::Array(items) => items.iter_mut().for_each(sort_bson),
        _ => (),
    }
}

fn sort_hjson(value: &mut serde_hjson::Value) {
    match value {
        serde_hjson::Value::Object(map) => {
            let mut entries: Vec<(String, serde_hjson::Value)> = take(map).into_iter().collect();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            *map = entries
                .into_iter()
                .map(|(key, mut value)| {
                    sort_hjson(&mut value);
                    (key, value)
                })
                .collect();
        }
        serde_hjson::Value::Array(items) => items.iter_mut().for_each(sort_hjson),
        _ => (),
    }
}

fn sort_json(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            map.sort_keys();
            map.values_mut().for_each(sort_json);
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(sort_json),
        _ => (),
    }
}

fn sort_plist(value: &mut plist::Value) {
    match value {
        plist::Value::Dictionary(dict) => {
            dict.sort_keys();
            dict.values_mut().for_each(sort_plist);
        }
        plist::Value::Array(items) => items.iter_mut().for_each(sort_plist),
        _ => (),
    }
}

fn sort_ron(value: &mut ron::Value) {
    match value {
        ron::Value::Map(map) => {
            let mut entries: Vec<(ron::Value, ron::Value)> = take(map).into_iter().collect();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            *map = entries
                .into_iter()
                .map(|(key, mut value)| {
                    sort_ron(&mut value);
                    (key, value)
                })
                .collect();
        }
        ron::Value::Seq(items) => items.iter_mut().for_each(sort_ron),
        ron::Value::Option(Some(inner)) => sort_ron(inner),
        _ => (),
    }
}

fn sort_toml(value: &mut toml::Value) {
    match value {
        toml::Value::Table(table) => {
            let mut entries: Vec<(String, toml::Value)> = take(table).into_iter().collect();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            *table = entries
                .into_iter()
                .map(|(key, mut value)| {
                    sort_toml(&mut value);
                    (key, value)
                })
                .collect();
        }
        toml::Value::Array(items) => items.iter_mut().for_each(sort_toml),
        _ => (),
    }
}

fn sort_yaml(value: &mut serde_yaml::Value) {
    match value {
        serde_yaml::Value::Mapping(mapping) => {
            let mut entries: Vec<(serde_yaml::Value, serde_yaml::Value)> =
                take(mapping).into_iter().collect();
            entries.sort_by(|(left, _), (right, _)| {
                left.partial_cmp(right).unwrap_or(Ordering::Equal)
            });
            *mapping = entries
                .into_iter()
                .map(|(key, mut value)| {
                    sort_yaml(&mut value);
                    (key, value)
                })
                .collect();
        }
        serde_yaml::Value::Sequence(items) => items.iter_mut().for_each(sort_yaml),
        serde_yaml::Value::Tagged(tagged) => sort_yaml(&mut tagged.value),
        _ => (),
    }
}

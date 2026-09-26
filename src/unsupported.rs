use std::mem::take;

use crate::{Format, Value};

/// Recursively drops values the target format has no representation for, so
/// that the conversion can go through instead of failing (or silently changing
/// the type).
///
/// Entries of objects and items of arrays are dropped alike, because toml
/// refuses to serialize a null wherever it sits. Returns how many values were
/// dropped, so that the caller can tell the user about the lost data.
pub fn ignore_unsupported(value: &mut Value, to: Format) -> usize {
    if to.supports_null() {
        return 0;
    }
    drop_nulls(value)
}

fn drop_nulls(value: &mut Value) -> usize {
    match value {
        Value::Bson(v) => drop_bson(v),
        Value::Csv(v) => drop_json(&mut v.items),
        Value::Dotenv(_) => 0,
        Value::Hjson(v) => drop_hjson(v),
        #[cfg(feature = "hocon")]
        Value::Hocon(v) => drop_json(&mut v.0),
        Value::Json(v) | Value::Json5(v) | Value::Toon(v) => drop_json(v),
        Value::Jsonl(v) => v.items.iter_mut().map(drop_json).sum(),
        Value::Plist(v) => drop_plist(v),
        Value::Ron(v) => drop_ron(v),
        Value::Toml(v) => drop_toml(v),
        Value::Xml(v) => drop_json(&mut v.0),
        Value::Yaml(v) => drop_yaml(v),
    }
}

fn drop_bson(value: &mut bson::Bson) -> usize {
    match value {
        bson::Bson::Document(doc) => {
            let entries: Vec<(String, bson::Bson)> = take(doc).into_iter().collect();
            let before = entries.len();
            let mut dropped = 0;
            *doc = entries
                .into_iter()
                .filter(|(_, item)| !is_bson_null(item))
                .map(|(key, mut item)| {
                    dropped += drop_bson(&mut item);
                    (key, item)
                })
                .collect();
            dropped + (before - doc.len())
        }
        bson::Bson::Array(items) => {
            let before = items.len();
            items.retain(|item| !is_bson_null(item));
            (before - items.len()) + items.iter_mut().map(drop_bson).sum::<usize>()
        }
        _ => 0,
    }
}

/// `Undefined` is deprecated, but it means the same "there is no value" as `Null`.
fn is_bson_null(value: &bson::Bson) -> bool {
    matches!(value, bson::Bson::Null | bson::Bson::Undefined)
}

fn drop_hjson(value: &mut serde_hjson::Value) -> usize {
    match value {
        serde_hjson::Value::Object(map) => {
            let entries: Vec<(String, serde_hjson::Value)> = take(map).into_iter().collect();
            let before = entries.len();
            let mut dropped = 0;
            *map = entries
                .into_iter()
                .filter(|(_, item)| !matches!(item, serde_hjson::Value::Null))
                .map(|(key, mut item)| {
                    dropped += drop_hjson(&mut item);
                    (key, item)
                })
                .collect();
            dropped + (before - map.len())
        }
        serde_hjson::Value::Array(items) => {
            let before = items.len();
            items.retain(|item| !matches!(item, serde_hjson::Value::Null));
            (before - items.len()) + items.iter_mut().map(drop_hjson).sum::<usize>()
        }
        _ => 0,
    }
}

fn drop_json(value: &mut serde_json::Value) -> usize {
    match value {
        serde_json::Value::Object(map) => {
            let before = map.len();
            map.retain(|_, item| !item.is_null());
            (before - map.len()) + map.values_mut().map(drop_json).sum::<usize>()
        }
        serde_json::Value::Array(items) => {
            let before = items.len();
            items.retain(|item| !item.is_null());
            (before - items.len()) + items.iter_mut().map(drop_json).sum::<usize>()
        }
        _ => 0,
    }
}

/// [`plist::Value`] has no null variant at all, so there is nothing to drop -
/// but a plist can still be the source of a conversion into another format
/// that has none either.
fn drop_plist(value: &mut plist::Value) -> usize {
    match value {
        plist::Value::Dictionary(dict) => dict.values_mut().map(drop_plist).sum(),
        plist::Value::Array(items) => items.iter_mut().map(drop_plist).sum(),
        _ => 0,
    }
}

fn drop_ron(value: &mut ron::Value) -> usize {
    match value {
        ron::Value::Map(map) => {
            let entries: Vec<(ron::Value, ron::Value)> = take(map).into_iter().collect();
            let before = entries.len();
            let mut dropped = 0;
            *map = entries
                .into_iter()
                .filter(|(_, item)| !is_ron_null(item))
                .map(|(key, mut item)| {
                    dropped += drop_ron(&mut item);
                    (key, item)
                })
                .collect();
            dropped + (before - map.len())
        }
        ron::Value::Seq(items) => {
            let before = items.len();
            items.retain(|item| !is_ron_null(item));
            (before - items.len()) + items.iter_mut().map(drop_ron).sum::<usize>()
        }
        ron::Value::Option(Some(inner)) => drop_ron(inner),
        _ => 0,
    }
}

/// Ron spells null either as a unit `()` or as an empty option `None`.
fn is_ron_null(value: &ron::Value) -> bool {
    matches!(value, ron::Value::Unit | ron::Value::Option(None))
}

/// Same as [`drop_plist`]: [`toml::Value`] has no null variant to drop.
fn drop_toml(value: &mut toml::Value) -> usize {
    match value {
        toml::Value::Table(table) => table.iter_mut().map(|(_, item)| drop_toml(item)).sum(),
        toml::Value::Array(items) => items.iter_mut().map(drop_toml).sum(),
        _ => 0,
    }
}

fn drop_yaml(value: &mut serde_yaml::Value) -> usize {
    match value {
        serde_yaml::Value::Mapping(mapping) => {
            let entries: Vec<(serde_yaml::Value, serde_yaml::Value)> =
                take(mapping).into_iter().collect();
            let before = entries.len();
            let mut dropped = 0;
            *mapping = entries
                .into_iter()
                .filter(|(_, item)| !item.is_null())
                .map(|(key, mut item)| {
                    dropped += drop_yaml(&mut item);
                    (key, item)
                })
                .collect();
            dropped + (before - mapping.len())
        }
        serde_yaml::Value::Sequence(items) => {
            let before = items.len();
            items.retain(|item| !item.is_null());
            (before - items.len()) + items.iter_mut().map(drop_yaml).sum::<usize>()
        }
        serde_yaml::Value::Tagged(tagged) => drop_yaml(&mut tagged.value),
        _ => 0,
    }
}

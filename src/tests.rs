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
        #[cfg(feature = "hocon")]
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

/// Same shape as [`get_test_value`], but with an extra `nothing: null` field for
/// formats that can represent `null`. Toml and Plist have no such representation.
fn get_test_value_with_null(format: Format, is_compact: bool) -> String {
    let value = match (format, is_compact) {
        (Format::Bson, _) => {
            "J\0\0\0\u{4}array\0\u{17}\0\0\0\u{2}0\0\u{2}\0\0\0a\0\u{2}1\0\u{2}\0\0\0b\0\0\u{8}boolean\0\0\u{a}nothing\0\u{12}the_answer\0*\0\0\0\0\0\0\0\0"
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
  nothing: null
  the_answer: 42
}"#
        }
        #[cfg(feature = "hocon")]
        (Format::Hocon, _) => {
            r#"
array: [a,b]
boolean: false
nothing: null
the_answer: 42
"#
        }
        (Format::Json, true) => {
            r#"{"array":["a","b"],"boolean":false,"nothing":null,"the_answer":42}"#
        }
        (Format::Json, false) => {
            r#"{
  "array": [
    "a",
    "b"
  ],
  "boolean": false,
  "nothing": null,
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
  nothing: null,
  the_answer: 42,
}"#
        }
        (Format::Jsonl, _) => unimplemented!("use raw data for tests"),
        (Format::Plist, _) => unimplemented!("plist has no null representation"),
        (Format::Ron, true) => {
            r#"{"array":["a","b"],"boolean":false,"nothing":(),"the_answer":42}"#
        }
        (Format::Ron, false) => {
            r#"{
    "array": [
        "a",
        "b",
    ],
    "boolean": false,
    "nothing": (),
    "the_answer": 42,
}"#
        }
        (Format::Toml, _) => unimplemented!("toml has no null representation"),
        (Format::Toon, _) => {
            r#"array[2]: a,b
boolean: false
nothing: null
the_answer: 42"#
        }
        (Format::Xml, _) => {
            r#"<root><array>a</array><array>b</array><boolean>false</boolean><nothing/><the_answer>42</the_answer></root>"#
        }
        (Format::Yaml, _) => {
            r#"array:
- a
- b
boolean: false
nothing: null
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

#[cfg(feature = "hocon")]
#[test]
fn test_convert_formats_hocon() {
    test_convert_formats(Format::Hocon, Format::Json, false);
}

/// Only pairs of formats that both support `null` (toml and plist do not).
#[rstest]
#[case(Format::Json, Format::Yaml, false)]
#[case(Format::Yaml, Format::Json, false)]
#[case(Format::Yaml, Format::Json, true)]
#[case(Format::Json, Format::Ron, true)]
#[case(Format::Json, Format::Ron, false)]
#[case(Format::Ron, Format::Json, true)]
#[case(Format::Json5, Format::Json, true)]
#[case(Format::Json, Format::Json5, true)]
#[case(Format::Json, Format::Json5, false)]
#[case(Format::Json5, Format::Json, false)]
#[case(Format::Json, Format::Bson, false)]
#[case(Format::Bson, Format::Json5, true)]
#[case(Format::Hjson, Format::Json, false)]
#[case(Format::Json, Format::Hjson, false)]
#[case(Format::Toon, Format::Yaml, false)]
#[case(Format::Toon, Format::Json, true)]
#[case(Format::Yaml, Format::Toon, false)]
#[case(Format::Yaml, Format::Toon, true)]
#[case(Format::Json, Format::Xml, true)]
fn test_convert_formats_null(
    #[case] from_format: Format,
    #[case] to_format: Format,
    #[case] is_compact: bool,
) {
    println!("{from_format:?} -> {to_format:?}. is_compact: {is_compact}");

    let input = get_test_value_with_null(from_format, is_compact);
    let expected_output = get_test_value_with_null(to_format, is_compact);

    let value = load_input(input.as_bytes(), from_format).unwrap();
    let output = String::from_utf8(dump_value(&value, to_format, is_compact).unwrap()).unwrap();

    assert_eq!(output, expected_output);
}

#[cfg(feature = "hocon")]
#[test]
fn test_convert_formats_null_hocon() {
    test_convert_formats_null(Format::Hocon, Format::Json, false);
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

#[rstest]
#[case(
    Format::Yaml,
    Format::Yaml,
    r#"b: 2
a: 1
nested:
  z: 1
  y:
  - q: 1
    p: 2
"#,
    r#"a: 1
b: 2
nested:
  y:
  - p: 2
    q: 1
  z: 1
"#,
    false
)]
#[case(
    Format::Json,
    Format::Json,
    r#"{"b":2,"a":1,"nested":{"z":1,"y":2},"array":[{"d":4,"c":3}]}"#,
    r#"{"a":1,"array":[{"c":3,"d":4}],"b":2,"nested":{"y":2,"z":1}}"#,
    true
)]
#[case(
    Format::Hjson,
    Format::Json,
    r#"{b:2,a:1,nested:{z:1,y:2}}"#,
    r#"{"a":1,"b":2,"nested":{"y":2,"z":1}}"#,
    true
)]
#[case(Format::Json5, Format::Json, r#"{b:2,a:1}"#, r#"{"a":1,"b":2}"#, true)]
#[case(
    Format::Toml,
    Format::Json,
    r#"b = 2
a = 1

[nested]
z = 1
y = 2
"#,
    r#"{"a":1,"b":2,"nested":{"y":2,"z":1}}"#,
    true
)]
#[case(
    Format::Ron,
    Format::Json,
    r#"{"b":2,"a":1,"nested":{"z":1,"y":2}}"#,
    r#"{"a":1,"b":2,"nested":{"y":2,"z":1}}"#,
    true
)]
#[case(
    Format::Bson,
    Format::Json,
    "\u{13}\0\0\0\u{10}b\0\u{2}\0\0\0\u{10}a\0\u{1}\0\0\0\0",
    r#"{"a":1,"b":2}"#,
    true
)]
#[case(
    Format::Plist,
    Format::Json,
    r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>b</key>
	<integer>2</integer>
	<key>a</key>
	<integer>1</integer>
</dict>
</plist>"#,
    r#"{"a":1,"b":2}"#,
    true
)]
#[case(
    Format::Toon,
    Format::Json,
    r#"b: 2
a: 1"#,
    r#"{"a":1,"b":2}"#,
    true
)]
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><b>2</b><a>1</a></root>"#,
    r#"{"a":1,"b":2}"#,
    true
)]
#[case(
    Format::Jsonl,
    Format::Xml,
    r#"{"b":2,"a":1}
{"d":4,"c":3}
"#,
    r#"<root><a>1</a><b>2</b></root>
<root><c>3</c><d>4</d></root>
"#,
    true
)]
#[case(
    Format::Csv,
    Format::Json,
    r#"b,a
2,1
"#,
    r#"[{"a":1,"b":2}]"#,
    true
)]
fn test_sort_keys(
    #[case] from_format: Format,
    #[case] to_format: Format,
    #[case] input: &str,
    #[case] expected_output: &str,
    #[case] is_compact: bool,
) {
    let mut value = load_input(input.as_bytes(), from_format).unwrap();
    sort_keys(&mut value);
    let output = String::from_utf8(dump_value(&value, to_format, is_compact).unwrap()).unwrap();

    assert_eq!(output, expected_output);
}

#[cfg(feature = "hocon")]
#[test]
fn test_sort_keys_hocon() {
    let mut value = load_input(b"b = 2\na = 1\n", Format::Hocon).unwrap();
    sort_keys(&mut value);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, r#"{"a":1,"b":2}"#);
}

#[cfg(feature = "hocon")]
#[test]
fn test_raw_convert_hocon() {
    test_raw_convert(
        Format::Hocon,
        Format::Json,
        r#"{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0}"#,
        r#"{"age":55000,"immortal":true,"name":"Gendalf the \"White\"","power":50.0}"#,
        true,
    );
}

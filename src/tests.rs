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
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><empty/></root>"#,
    r#"{"empty":null}"#,
    true
)]
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><empty></empty></root>"#,
    r#"{"empty":null}"#,
    true
)]
#[case(Format::Xml, Format::Json, r#"<root/>"#, r#"{}"#, true)]
#[case(Format::Xml, Format::Json, r#"<root></root>"#, r#"{}"#, true)]
#[case(
    Format::Json,
    Format::Xml,
    r#"{"empty":null}"#,
    r#"<root><empty/></root>"#,
    true
)]
// xml: attributes of an open tag land next to the tag's own value
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><item a="1" b="x">text</item></root>"#,
    r#"{"item":[{"@a":1,"@b":"x"},"text"]}"#,
    true
)]
// xml: attributes of a self-closing tag become the tag's value
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><item a="1" b="x"/></root>"#,
    r#"{"item":{"@a":1,"@b":"x"}}"#,
    true
)]
// xml: repeated tags collapse into a single array
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><a>1</a><a>2</a><a>3</a></root>"#,
    r#"{"a":[1,2,3]}"#,
    true
)]
// xml: cdata is read as plain text
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><a><![CDATA[1 < 2 & 3]]></a></root>"#,
    r#"{"a":"1 < 2 & 3"}"#,
    true
)]
// xml: text next to child tags is kept under `#text`
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><a><b>1</b>tail</a></root>"#,
    r##"{"a":{"b":1,"#text":"tail"}}"##,
    true
)]
// xml: declaration and comments are skipped
#[case(
    Format::Xml,
    Format::Json,
    r#"<?xml version="1.0" encoding="UTF-8"?><!-- comment --><root><a>1</a></root>"#,
    r#"{"a":1}"#,
    true
)]
// xml: a root tag named something other than `root` is kept as a key
#[case(
    Format::Xml,
    Format::Json,
    r#"<data><a>1</a></data>"#,
    r#"{"data":{"a":1}}"#,
    true
)]
// xml: floats are parsed, non-finite ones stay strings
#[case(
    Format::Xml,
    Format::Json,
    r#"<root><x>1.5</x><y>inf</y></root>"#,
    r#"{"x":1.5,"y":"inf"}"#,
    true
)]
// json -> xml: `@` keys become attributes, `#text` becomes the tag body
#[case(
    Format::Json,
    Format::Xml,
    r##"{"item":{"@a":"1","#text":"hello & <bye>"}}"##,
    r#"<root><item a="1">hello &amp; &lt;bye&gt;</item></root>"#,
    true
)]
// json -> xml: an empty object has no content, so the tag is self-closing
#[case(
    Format::Json,
    Format::Xml,
    r#"{"item":{}}"#,
    r#"<root><item/></root>"#,
    true
)]
#[case(Format::Json, Format::Xml, r#"{}"#, r#"<root/>"#, true)]
// json -> xml: a scalar root has no tag to go into
#[case(Format::Json, Format::Xml, "42", "42", true)]
#[case(Format::Json, Format::Xml, "true", "true", true)]
#[case(Format::Json, Format::Xml, r#""a & b""#, "a &amp; b", true)]
#[case(Format::Json, Format::Xml, "null", "", true)]
// csv: null is an empty cell, which is what `load_csv` reads back as null
#[case(
    Format::Json,
    Format::Csv,
    r#"[{"a":1,"b":null},{"a":2,"b":3}]"#,
    "a,b\n1,\n2,3\n",
    true
)]
fn test_raw_convert(
    #[case] from_format: Format,
    #[case] to_format: Format,
    #[case] input: &str,
    #[case] expected_output: &str,
    #[case] is_compact: bool,
) {
    println!("checking test, input {:?}", input);
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

#[rstest]
#[case(Format::Bson, true)]
#[case(Format::Csv, false)]
#[case(Format::Json, false)]
#[case(Format::Xml, false)]
fn test_is_binary(#[case] format: Format, #[case] expected: bool) {
    assert_eq!(format.is_binary(), expected);
}

#[rstest]
// nothing to read at all
#[case(Format::Xml, "")]
// no tags, so there is no root element
#[case(Format::Xml, "just text")]
// mismatched closing tag
#[case(Format::Xml, "<root><a></b></root>")]
// root tag is never closed
#[case(Format::Xml, "<root><a>1</a>")]
#[case(Format::Jsonl, "{not json}")]
#[case(Format::Csv, "a,b\n\"unterminated")]
#[case(Format::Json, "{")]
fn test_load_input_errors(#[case] format: Format, #[case] input: &str) {
    assert!(load_input(input.as_bytes(), format).is_err());
}

/// Csv and jsonl both need a root array of objects.
#[rstest]
#[case(Format::Csv, r#"{"a":1}"#)]
#[case(Format::Csv, r#"[1,2]"#)]
#[case(Format::Jsonl, r#"{"a":1}"#)]
fn test_dump_value_errors(#[case] to_format: Format, #[case] input: &str) {
    let value = load_input(input.as_bytes(), Format::Json).unwrap();
    assert!(dump_value(&value, to_format, true).is_err());
}

/// Every `sort_*` implementation handles arrays in a separate branch.
#[rstest]
#[case(Format::Bson)]
#[case(Format::Hjson)]
#[case(Format::Json)]
#[case(Format::Plist)]
#[case(Format::Ron)]
#[case(Format::Toml)]
#[case(Format::Yaml)]
fn test_sort_keys_inside_arrays(#[case] format: Format) {
    let source = r#"{"a":1,"b":[{"d":4,"c":3}]}"#;

    let json = load_input(source.as_bytes(), Format::Json).unwrap();
    let encoded = dump_value(&json, format, true).unwrap();

    let mut value = load_input(&encoded, format).unwrap();
    sort_keys(&mut value);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, r#"{"a":1,"b":[{"c":3,"d":4}]}"#);
}

/// Ron options wrap a value that has to be sorted too.
#[test]
fn test_sort_keys_ron_option() {
    let mut value = load_input(br#"{"b":Some({"z":1,"y":2}),"a":1}"#, Format::Ron).unwrap();
    sort_keys(&mut value);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, r#"{"a":1,"b":{"y":2,"z":1}}"#);
}

/// Same for the value behind a yaml tag.
#[test]
fn test_sort_keys_yaml_tagged() {
    let mut value = load_input(b"b: !tag\n  z: 1\n  y: 2\na: 1\n", Format::Yaml).unwrap();
    sort_keys(&mut value);
    let output = String::from_utf8(dump_value(&value, Format::Yaml, false).unwrap()).unwrap();

    assert_eq!(output, "a: 1\nb: !tag\n  y: 2\n  z: 1\n");
}

/// Nulls at the top level, inside a nested object and inside an array.
const WITH_NULLS: &str = r#"{"a":1,"b":null,"c":{"d":null,"e":2},"arr":[1,null,2]}"#;
const WITHOUT_NULLS: &str = r#"{"a":1,"c":{"e":2},"arr":[1,2]}"#;

#[rstest]
#[case(Format::Toml, false)]
#[case(Format::Plist, false)]
#[case(Format::Bson, true)]
#[case(Format::Csv, true)]
#[case(Format::Json, true)]
#[case(Format::Xml, true)]
#[case(Format::Yaml, true)]
fn test_supports_null(#[case] format: Format, #[case] expected: bool) {
    assert_eq!(format.supports_null(), expected);
}

/// Messages about a format use the name from `--from`/`--to`, not the variant.
#[rstest]
#[case(Format::Toml, "toml")]
#[case(Format::Json5, "json5")]
fn test_format_display(#[case] format: Format, #[case] expected: &str) {
    assert_eq!(format.to_string(), expected);
}

/// Dropping happens only for the formats that can't hold a null.
#[rstest]
#[case(Format::Toml, 3, WITHOUT_NULLS)]
#[case(Format::Plist, 3, WITHOUT_NULLS)]
#[case(Format::Bson, 0, WITH_NULLS)]
#[case(Format::Csv, 0, WITH_NULLS)]
#[case(Format::Json, 0, WITH_NULLS)]
#[case(Format::Xml, 0, WITH_NULLS)]
#[case(Format::Yaml, 0, WITH_NULLS)]
fn test_ignore_unsupported(
    #[case] to_format: Format,
    #[case] expected_skipped: usize,
    #[case] expected_output: &str,
) {
    let mut value = load_input(WITH_NULLS.as_bytes(), Format::Json).unwrap();
    assert_eq!(ignore_unsupported(&mut value, to_format), expected_skipped);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, expected_output);
}

/// The same set of nulls, reached through every source representation.
#[rstest]
#[case(Format::Bson, WITHOUT_NULLS)]
#[case(Format::Hjson, WITHOUT_NULLS)]
#[case(Format::Json, WITHOUT_NULLS)]
#[case(Format::Json5, WITHOUT_NULLS)]
// ron keeps map keys sorted, so the round-trip reorders them
#[case(Format::Ron, r#"{"a":1,"arr":[1,2],"c":{"e":2}}"#)]
#[case(Format::Toon, WITHOUT_NULLS)]
#[case(Format::Xml, WITHOUT_NULLS)]
#[case(Format::Yaml, WITHOUT_NULLS)]
fn test_ignore_unsupported_sources(#[case] format: Format, #[case] expected_output: &str) {
    let json = load_input(WITH_NULLS.as_bytes(), Format::Json).unwrap();
    let encoded = dump_value(&json, format, true).unwrap();

    let mut value = load_input(&encoded, format).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 3);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, expected_output);
}

#[test]
fn test_ignore_unsupported_csv() {
    let mut value = load_input(b"a,b\n1,\n", Format::Csv).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 1);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, r#"[{"a":1}]"#);
}

#[test]
fn test_ignore_unsupported_jsonl() {
    let input = "{\"a\":1,\"b\":null}\n{\"c\":null,\"d\":2}\n";
    let mut value = load_input(input.as_bytes(), Format::Jsonl).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 2);
    let output = String::from_utf8(dump_value(&value, Format::Xml, true).unwrap()).unwrap();

    assert_eq!(output, "<root><a>1</a></root>\n<root><d>2</d></root>\n");
}

/// Toml and plist have no null variant to drop, but the walk still has to
/// reach nested values without breaking them.
#[rstest]
#[case(Format::Plist)]
#[case(Format::Toml)]
fn test_ignore_unsupported_without_null_variant(#[case] format: Format) {
    let source = r#"{"a":1,"arr":[{"f":3}],"c":{"e":2}}"#;
    let json = load_input(source.as_bytes(), Format::Json).unwrap();
    let encoded = dump_value(&json, format, true).unwrap();

    let mut value = load_input(&encoded, format).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 0);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, source);
}

/// Ron spells null either as a unit or as an empty option.
#[test]
fn test_ignore_unsupported_ron_unit_and_none() {
    let mut value =
        load_input(br#"{"a":(),"b":None,"c":Some(1),"d":[(),2]}"#, Format::Ron).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 3);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, r#"{"c":1,"d":[2]}"#);
}

#[test]
fn test_ignore_unsupported_yaml_tagged() {
    let mut value = load_input(b"a: !tag\n  b: null\n  c: 2\n", Format::Yaml).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 1);
    let output = String::from_utf8(dump_value(&value, Format::Yaml, false).unwrap()).unwrap();

    assert_eq!(output, "a: !tag\n  c: 2\n");
}

/// A null root has no container to be removed from, so the dump still fails.
#[test]
fn test_ignore_unsupported_keeps_root_null() {
    let mut value = load_input(b"null", Format::Json).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 0);

    assert!(dump_value(&value, Format::Toml, true).is_err());
}

/// The case the option exists for.
#[test]
fn test_ignore_unsupported_enables_toml() {
    let mut value = load_input(WITH_NULLS.as_bytes(), Format::Json).unwrap();
    assert!(dump_value(&value, Format::Toml, true).is_err());

    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 3);
    let output = String::from_utf8(dump_value(&value, Format::Toml, true).unwrap()).unwrap();

    assert_eq!(output, "a = 1\narr = [1, 2]\n\n[c]\ne = 2\n");
}

#[cfg(feature = "hocon")]
#[test]
fn test_ignore_unsupported_hocon() {
    let mut value = load_input(b"a = 1\nb = null\n", Format::Hocon).unwrap();
    assert_eq!(ignore_unsupported(&mut value, Format::Toml), 1);
    let output = String::from_utf8(dump_value(&value, Format::Json, true).unwrap()).unwrap();

    assert_eq!(output, r#"{"a":1}"#);
}

#[cfg(feature = "hocon")]
#[rstest]
#[case(true, r#"{"a":1,"b":[2,3]}"#)]
#[case(
    false,
    r#"{
  "a": 1,
  "b": [
    2,
    3
  ]
}"#
)]
fn test_dump_hocon(#[case] is_compact: bool, #[case] expected: &str) {
    let value = load_input(br#"{"a":1,"b":[2,3]}"#, Format::Json).unwrap();
    let output = String::from_utf8(dump_value(&value, Format::Hocon, is_compact).unwrap()).unwrap();

    assert_eq!(output, expected);
}

#[cfg(feature = "hocon")]
#[test]
fn test_load_hocon_errors() {
    assert!(load_input(b"a = ${undefined}", Format::Hocon).is_err());
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

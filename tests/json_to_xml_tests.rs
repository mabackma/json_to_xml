use json_to_xml::generate_xml::json_to_xml;

#[test]
fn test_basic_conversion() {
    let json = r#"{
        "@xmlns:pr": "http://test/person",
        "person": {
            "name": "Alice",
            "@id": "42"
        }
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Root xmlns:pr=\"http://test/person\">"));
    assert!(xml.contains("<Person id=\"42\">"));
    assert!(xml.contains("<Name>Alice</Name>"));
    assert!(xml.contains("</Person>"));
    assert!(xml.contains("</Root>"));
}

#[test]
fn test_array_conversion() {
    let json = r#"{
        "items": [
            {"value": "A"},
            {"value": "B"}
        ]
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Items>"));
    assert!(xml.contains("<Value>A</Value>"));
    assert!(xml.contains("<Value>B</Value>"));
    assert!(xml.matches("<Items>").count() >= 2);
}

#[test]
fn test_empty_object() {
    let json = r#"{
        "empty": {}
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Empty/>"));
}

#[test]
fn test_empty_array() {
    let json = r#"{
        "list": []
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<List></List>") || xml.contains("<List/>"));
}

#[test]
fn test_number_value() {
    let json = r#"{
        "age": 99
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Age>99</Age>"));
}

#[test]
fn test_text_key_handling() {
    let json = r#"{
        "person": {
            "$text": "hello"
        }
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Person>hello</Person>"));
}

#[test]
fn test_top_level_attributes_trigger_root_tag() {
    let json = r#"{
        "@xmlns:x": "http://x",
        "item": "value"
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Root xmlns:x=\"http://x\">"));
    assert!(xml.contains("<Item>value</Item>"));
    assert!(xml.contains("</Root>"));
}

#[test]
fn test_nested_attributes() {
    let json = r#"{
        "outer": {
            "@type": "x",
            "inner": {
                "@id": "5",
                "value": "ok"
            }
        }
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Outer type=\"x\">"));
    assert!(xml.contains("<Inner id=\"5\">"));
    assert!(xml.contains("<Value>ok</Value>"));
}

#[test]
fn test_array_of_objects_with_attributes() {
    let json = r#"{
        "items": [
            {"@id": "1", "value": "A"},
            {"@id": "2", "value": "B"}
        ]
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("id=\"1\""));
    assert!(xml.contains("id=\"2\""));
    assert!(xml.contains("<Value>A</Value>"));
    assert!(xml.contains("<Value>B</Value>"));
}

#[test]
fn test_mixed_types_array() {
    let json = r#"{
        "data": [
            "A",
            123,
            {"x": "y"}
        ]
    }"#;

    let xml = json_to_xml(json, "Root");

    assert!(xml.contains("<Data>A</Data>"));
    assert!(xml.contains("<Data>123</Data>"));
    assert!(xml.contains("<X>y</X>"));
}

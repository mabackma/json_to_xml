use json_to_xml::generate_xml::{json_to_xml};
use json_to_xml::error::ConversionError;

#[test]
fn test_basic_conversion() -> Result<(), ConversionError> {
    let json = r#"{
        "@xmlns:pr": "http://test/person",
        "person": {
            "name": "Alice",
            "@id": "42"
        }
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Root xmlns:pr=\"http://test/person\">"));
    assert!(xml.contains("<Person id=\"42\">"));
    assert!(xml.contains("<Name>Alice</Name>"));
    assert!(xml.contains("</Person>"));
    assert!(xml.contains("</Root>"));
    Ok(())
}

#[test]
fn test_array_conversion() -> Result<(), ConversionError> {
    let json = r#"{
        "items": [
            {"value": "A"},
            {"value": "B"}
        ]
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Items>"));
    assert!(xml.contains("<Value>A</Value>"));
    assert!(xml.contains("<Value>B</Value>"));
    assert!(xml.matches("<Items>").count() >= 2);
    Ok(())
}

#[test]
fn test_empty_object() -> Result<(), ConversionError> {
    let json = r#"{
        "empty": {}
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Empty/>"));
    Ok(())
}

#[test]
fn test_empty_array() -> Result<(), ConversionError> {
    let json = r#"{
        "list": []
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<List>") && xml.contains("</List>"));
    Ok(())
}

#[test]
fn test_number_value() -> Result<(), ConversionError> {
    let json = r#"{
        "age": 99
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Age>99</Age>"));
    Ok(())
}

#[test]
fn test_text_key_handling() -> Result<(), ConversionError> {
    let json = r#"{
        "person": {
            "$text": "hello"
        }
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Person>hello</Person>"));
    Ok(())
}

#[test]
fn test_top_level_attributes_trigger_root_tag() -> Result<(), ConversionError> {
    let json = r#"{
        "@xmlns:x": "http://x",
        "item": "value"
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Root xmlns:x=\"http://x\">"));
    assert!(xml.contains("<Item>value</Item>"));
    assert!(xml.contains("</Root>"));
    Ok(())
}

#[test]
fn test_nested_attributes() -> Result<(), ConversionError> {
    let json = r#"{
        "outer": {
            "@type": "x",
            "inner": {
                "@id": "5",
                "value": "ok"
            }
        }
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Outer type=\"x\">"));
    assert!(xml.contains("<Inner id=\"5\">"));
    assert!(xml.contains("<Value>ok</Value>"));
    Ok(())
}

#[test]
fn test_array_of_objects_with_attributes() -> Result<(), ConversionError> {
    let json = r#"{
        "items": [
            {"@id": "1", "value": "A"},
            {"@id": "2", "value": "B"}
        ]
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("id=\"1\""));
    assert!(xml.contains("id=\"2\""));
    assert!(xml.contains("<Value>A</Value>"));
    assert!(xml.contains("<Value>B</Value>"));
    Ok(())
}

#[test]
fn test_mixed_types_array() -> Result<(), ConversionError> {
    let json = r#"{
        "data": [
            "A",
            123,
            {"x": "y"}
        ]
    }"#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<DataItem>A</DataItem>"));
    assert!(xml.contains("<DataItem>123</DataItem>"));
    assert!(xml.contains("<X>y</X>"));
    Ok(())
}

#[test]
fn test_deep_nested_mixed_arrays() -> Result<(), ConversionError> {
    let json = r#"
    {
        "root": {
            "items": [
                {
                    "inner1": [
                        "x",
                        42,
                        { "b": 4 }
                    ],
                    "inner2": [
                        { "a": "1" },
                        "x",
                        420
                    ],
                    "inner3": { "c": [40, {"d": 50}, 60] }
                }
            ]
        }
    }
    "#;

    let xml = json_to_xml(json)?;

    assert!(xml.contains("<Inner1Item>x</Inner1Item>"));
    assert!(xml.contains("<Inner1Item>42</Inner1Item>"));
    assert!(xml.contains("<B>4</B>"));

    assert!(xml.contains("<A>1</A>"));
    assert!(xml.contains("<Inner2Item>x</Inner2Item>"));
    assert!(xml.contains("<Inner2Item>420</Inner2Item>"));

    assert!(xml.contains("<CItem>40</CItem>"));
    assert!(xml.contains("<D>50</D>"));
    assert!(xml.contains("<CItem>60</CItem>"));
    Ok(())
}
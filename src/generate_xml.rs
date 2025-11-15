use crate::xml_utils::{write_declaration, write_comment, write_start_tag, write_empty_tag, write_end_tag, write_content};

use std::fs;
use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart};
use serde_json::{Value, Map, from_str};
use std::collections::HashMap;
use std::io::Cursor;

/// # Convert JSON to XML.
/// 
/// # Example
/// 
/// ```rust
/// use json_to_xml::generate_xml::json_to_xml;
/// 
/// let json_string = r#"
/// {
///     "@xmlns:addr": "http://standards.fi/schemas/personData/addresses",
///     "@xmlns:pr": "http://standards.fi/schemas/personData/person",
///     "person": {
///         "name": "John Doe",
///         "age": "30",
///         "@id": "1234",
///         "addresses": [
///             {
///                 "street": "123 Main St",
///                 "city": "Springfield",
///                 "@type": "primary"
///             },
///             {
///                 "street": "456 Oak Ave",
///                 "city": "Shelbyville",
///                 "@type": "secondary"
///             }
///         ]
///     }
/// }
/// "#;
/// 
/// let xml_string = json_to_xml(&json_string, "People");
/// 
/// println!("{}", xml_string);
/// ```
/// 
/// ## Expected Output (XML):
/// 
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// <!--Generated with json_to_xml 0.1.0-->
/// <People xmlns:pr="http://standards.fi/schemas/personData/person" xmlns:addr="http://standards.fi/schemas/personData/addresses">
///   <Person id="1234">
///     <Addresses type="primary">
///       <City>Springfield</City>
///       <Street>123 Main St</Street>
///     </Addresses>
///     <Addresses type="secondary">
///       <City>Shelbyville</City>
///       <Street>456 Oak Ave</Street>
///     </Addresses>
///     <Age>30</Age>
///     <Name>John Doe</Name>
///   </Person>
/// </People>
/// ```
/// 
/// ## Parameters:
/// - `json_string`: The input JSON string to be converted into XML. It can contain objects, arrays, and strings.
/// - `root`: The name of the root. It will become the root element of the XML if the JSON contains top-level `@` attributes.
///
/// ## Returns:
/// A string containing the XML representation of the input JSON, including necessary XML attributes.
///
/// ## Notes:
/// - This function works recursively to handle nested structures and arrays.
/// - Attributes are prefixed with `@` in the JSON input and are converted to XML attributes.
/// - The order of attributes in the XML elements may differ.
/// - The root start and end tags will be included only if the top-level JSON object contains `@` attributes.
pub fn json_to_xml(json_string: &str, root: &str) -> String {
    let json_value: Value = from_str(&json_string).unwrap();
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2); // 2-space indentation

    // Write XML header
    write_declaration(&mut writer, "1.0", Some("UTF-8"));
    
    // Write metadata comment
    let mut version = get_dependency_version("Cargo.toml").unwrap_or("0.0.0".to_string());
    version = format!("Generated with json_to_xml {}", version);
    write_comment(&mut writer, &version);
    
    // Write the XML
    create_xml_element(&json_value, &mut writer, root);

    // Write the closing tag if there was a top level attribute
    if has_top_level_attributes(&json_value) {
        write_end_tag(&mut writer, &BytesEnd::new(root));
    }

    String::from_utf8(writer.into_inner().into_inner()).expect("Failed to convert to UTF-8")
}

/// Helper function to get json_to_xml version from the Cargo.toml file
pub fn get_dependency_version(file_path: &str) -> Option<String> {
    let content = fs::read_to_string(file_path).expect("Unable to read the file");
    let toml: Value = toml::de::from_str(&content).expect("Unable to parse TOML");

    toml.get("dependencies")
        .and_then(|deps| deps.get("json_to_xml"))
        .and_then(|dep| {
            if dep.is_object() {
                dep.get("version").and_then(|v| v.as_str()).map(|s| s.to_string())
            } else {
                dep.as_str().map(|s| s.to_string())
            }
        })
}

/// Recursively create XML elements from JSON data
fn create_xml_element(
    json_data: &Value, 
    writer: &mut Writer<Cursor<Vec<u8>>>, 
    parent_tag: &str
) {
    match json_data {
        // Handle objects
        Value::Object(map) => {
            handle_object(writer, map, parent_tag);
        },

        // Handle arrays by processing each item inside the array
        Value::Array(arr) => {
            handle_array(writer, arr, parent_tag);
        },

        // Handle strings as text content
        Value::String(s) => {
            write_content(writer, s);
        },

        // Handle numbers as text content
        Value::Number(num) => {
            write_content(writer, &num.to_string());
        }

        // Handle booleans
        Value::Bool(b) => {
            write_content(writer, &b.to_string());
        }

        // Handle Null values
        Value::Null => {
            write_empty_tag(writer, &BytesStart::new(parent_tag));
        }
    }
}

fn handle_object(writer: &mut Writer<Cursor<Vec<u8>>>, map: &Map<String, Value>, parent_tag: &str) {
    let parent_tag = capitalize_word(parent_tag);
    let mut element = BytesStart::new(parent_tag);

    // Extract attributes
    let attributes: HashMap<_, _> = map
        .iter()
        .filter(|(key, _)| key.starts_with("@"))
        .map(|(key, value)| (&key[1..], value))
        .collect();

    // Add attributes to the element
    for (key, value) in &attributes {
        if let Some(value_str) = value.as_str() {
            element.push_attribute((*key, value_str));
        }
    }

    // Write start tag with attributes, if any
    if !attributes.is_empty() {
        write_start_tag(writer, &element);
    }

    if map.contains_key("$text") {
        let text_content = map.get("$text").unwrap().as_str().unwrap();
        write_content(writer, text_content);
    }

    // Process key-value pairs
    for (key, value) in map {
        // Reset the element for the next iteration
        let key_tag = capitalize_word(key);
        element = BytesStart::new(key_tag.clone());

        // Write self-closing tag if the object is empty
        if value.is_object() && value.as_object().unwrap().is_empty() {
            write_empty_tag(writer, &element);
            continue;
        }

        // Skip attributes
        if key.starts_with("@") || key == "$text" {
            continue;
        } else {
            // Write the start tag if the value is not an attribute or an array with a first key as an attribute
            if !(is_attribute_key(value) || is_array_with_attribute_key(value)) {
                write_start_tag(writer, &element);
            }

            // Recursively process nested elements
            create_xml_element(value, writer, key);
            
            // Write the closing tag if the value is not an array
            if !value.is_array() {
                write_end_tag(writer, &BytesEnd::new(key_tag));
            }
        }
    }
}

fn handle_array(writer: &mut Writer<Cursor<Vec<u8>>>, arr: &Vec<Value>, parent_tag: &str) {
    let mut parent_tag = capitalize_word(parent_tag);
    
    // Handle empty array
    if arr.is_empty() {
        write_empty_tag(writer, &BytesStart::new(&parent_tag));
        return;
    }

    let original_tag = parent_tag.clone();

    for (i, value) in arr.iter().enumerate() {
        if value.is_object() {
            handle_object_array(writer, i, value, &parent_tag);
        } else {
            parent_tag = "Item".to_string();

            write_start_tag(writer, &BytesStart::new(&parent_tag));
            create_xml_element(value, writer, &original_tag);
            write_end_tag(writer, &BytesEnd::new(&parent_tag));

            if i == arr.len() - 1 {
                write_end_tag(writer, &BytesEnd::new(&original_tag));
            }
        }
    }
}

fn handle_object_array(
    writer: &mut Writer<Cursor<Vec<u8>>>, 
    index: usize, 
    value: &Value, 
    parent_tag: &str
) {
    let first_key = value.as_object().unwrap().keys().next().unwrap();

    // Write the start tag for all non-attribute elements, skipping the first one
    if !first_key.starts_with("@") && index > 0 {
        write_start_tag(writer, &BytesStart::new(parent_tag));
    } 

    create_xml_element(value, writer, &parent_tag);

    write_end_tag(writer, &BytesEnd::new(parent_tag));
}

// Check if json has top-level attributes
fn has_top_level_attributes(json: &Value) -> bool {
    if let Value::Object(map) = json {
        map.keys().any(|key| key.starts_with('@'))
    } else {
        false
    }
}

// Check if any key of the object is an attribute
fn is_attribute_key(value: &Value) -> bool {
    value.is_object()
        && value.as_object()
            .unwrap()
            .keys()
            .any(|key| key.starts_with("@")) // Check if any key is an attribute
}

// Check if any key of the first object in array is an attribute
fn is_array_with_attribute_key(value: &Value) -> bool {
    value.is_array()
        && value.as_array()
            .unwrap()
            .first()
            .map(|v| is_attribute_key(v))
            .unwrap_or(false)
}

// Capitalizes the first letter of a word.
fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
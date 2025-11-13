use std::fs;
use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, BytesDecl, Event};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Cursor;

/// # Convert JSON to XML.
/// 
/// # Example
/// 
/// ```rust
/// use schema_generator::generate_xml::json_to_xml;
/// 
/// let json_data = r#"
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
/// let json: serde_json::Value = serde_json::from_str(json_data).unwrap();
/// 
/// let xml_output = json_to_xml(&json, "People");
/// 
/// println!("{}", xml_output);
/// ```
/// 
/// ## Expected Output (XML):
/// 
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// <!--Generated with schema_generator 0.1.0-->
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
/// - `json_value`: The input JSON value to be converted into XML. It can contain objects, arrays, and strings.
/// - `root`: The name of the root element in the XML document.
///
/// ## Returns:
/// A string containing the XML representation of the input JSON, including necessary XML namespaces and attributes.
///
/// ## Notes:
/// - This function works recursively to handle nested structures and arrays.
/// - Attributes are prefixed with `@` in the JSON input and are converted to XML attributes.
/// - The order of attributes in the XML elements may differ.
pub fn json_to_xml(json_value: &Value, root: &str) -> String {

    // Create the writer
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2); // 2-space indentation

    // Write XML header
    writer
        .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
        .expect("Unable to write XML declaration");
    
    // Write metadata comment
    let version = get_dependency_version("Cargo.toml").unwrap_or("0.0.0".to_string());
    writer
        .write_event(
            Event::Comment(BytesText::new(&format!(
            "Generated with json_to_xml {}", 
            version
        ))))
        .expect("Unable to write comment");
    
    create_xml_element(json_value, &mut writer, root);

    // Write the closing tag
    writer
        .write_event(Event::End(BytesEnd::new(root)))
        .expect("Unable to write end tag"); 

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

// Recursively create XML elements from JSON data
fn create_xml_element(
    json_data: &Value, 
    writer: &mut Writer<Cursor<Vec<u8>>>, 
    parent_tag: &str
) {
    match json_data {

        // Handle objects
        Value::Object(map) => {
            let mut parent_tag = parent_tag.to_string();
            parent_tag = capitalize_word(&parent_tag);

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
                writer
                    .write_event(Event::Start(element.to_owned()))
                    .expect("Unable to write start tag");
            }

            if map.contains_key("$text") {
                let text_content = map.get("$text").unwrap().as_str().unwrap();
                writer
                    .write_event(Event::Text(BytesText::new(&text_content)))
                    .expect("Unable to write text");
            }

            // Process key-value pairs
            for (key, value) in map {
                // Reset the element for the next iteration				  
                let mut key_tag = key.to_string();
                key_tag = capitalize_word(&key_tag);

                element = BytesStart::new(key_tag.clone());

                // Write self-closing tag if the object is empty
                if value.is_object() && value.as_object().unwrap().is_empty() {
                    writer
                        .write_event(Event::Empty(element.to_owned()))
                        .expect("Unable to write self-closing tag");

                    continue;
                }

                // Skip attributes
                if key.starts_with("@") || key == "$text" {
                    continue;
                } else {

                    // Write the start tag if the value is not an attribute or an array with a first key as an attribute
                    if !(is_attribute_key(value) || is_array_with_attribute_key(value)) {
                        writer
                            .write_event(Event::Start(element.to_owned()))
                            .expect("Unable to write start tag");
                    }

					// Recursively process nested elements
					create_xml_element(value, writer, key);
					
                    // Write the closing tag if the value is not an array
                    if !value.is_array() {
                        writer
                            .write_event(Event::End(BytesEnd::new(key_tag)))
                            .expect("Unable to write end tag");
                    }
                }
            }
        },

        // Handle arrays by processing each item inside the array
        Value::Array(arr) => {
            let mut parent_tag = parent_tag.to_string();
            parent_tag = capitalize_word(&parent_tag);
            
            for (i, value) in arr.iter().enumerate() {

                // Get the first key of the object 
                if value.is_object() {
                    let first_key = value.as_object().unwrap().keys().next().unwrap();

                    // Write the start tag for all non-attribute elements, skipping the first one
                    if !first_key.starts_with("@") && i > 0 {
                        writer
                            .write_event(Event::Start(BytesStart::new(&parent_tag)))
                            .expect("Unable to write start tag"); 
                    } 
                }

                // Process each element of the array as a separate XML tag
                create_xml_element(value, writer, &parent_tag);

                // Write the closing tag
                writer
                    .write_event(Event::End(BytesEnd::new(&parent_tag)))
                    .expect("Unable to write end tag");
            }
        },

        // Handle strings as text content
        Value::String(s) => {
            writer
                .write_event(Event::Text(BytesText::new(s)))
                .expect("Unable to write text");
        },

        // Handle number as text content
        Value::Number(num) => {
            let num_str = if num.is_i64() {
                num.as_i64().unwrap().to_string()
            } else if num.is_f64() {
                format!("{}", num.as_f64().unwrap()) // Convert float properly
            } else {
                String::new()
            };

            writer
                .write_event(Event::Text(BytesText::new(&num_str)))
                .expect("Unable to write number");
        }

        // Skip unsupported types
        _ => {} 
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

/// Capitalizes the first letter of a word.
fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
use crate::error::ConversionError;
use crate::xml_utils::{write_declaration, write_comment, write_start_tag, write_empty_tag, write_end_tag, write_content};

use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart};
use serde_json::{Value, Map, from_str};
use std::collections::HashMap;
use std::io::Cursor;
use std::fs;
use toml::de::from_str as toml_from_str;
/// # Convert JSON to XML with a default "Root" element.
///
/// This is a convenience function that calls `json_to_xml_with_root` with "Root" as the default root element name.
///
/// # Example
///
/// ```rust
/// use json_to_xml::generate_xml::json_to_xml;
///
/// let json_string = r#"
/// {
///     "book": {
///         "@isbn": "978-3-16-148410-0",
///         "title": "The Rust Programming Language",
///         "author": "Steve Klabnik and Carol Nichols"
///     }
/// }
/// "#;
///
/// let xml_string = json_to_xml(&json_string).unwrap();
///
/// println!("{}", xml_string);
/// ```
///
/// ## Expected Output (XML):
///
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// <!--Generated with json_to_xml 0.1.7-->
/// <Root>
///   <Book isbn="978-3-16-148410-0">
///     <Author>Steve Klabnik and Carol Nichols</Author>
///     <Title>The Rust Programming Language</Title>
///   </Book>
/// </Root>
/// ```
pub fn json_to_xml(json_string: &str) -> Result<String, ConversionError> {
    json_to_xml_with_root(json_string, "Root")
}

/// # Convert JSON to XML with a custom root element.
///
/// This function takes a JSON string and a specified root element name and converts it into an XML string.
/// It processes JSON objects, arrays, and primitive values recursively.
/// Attributes in JSON (prefixed with `@`) are converted to XML attributes.
/// All XML tags are capitalized.
///
/// # Example
///
/// ```rust
/// use json_to_xml::generate_xml::json_to_xml_with_root;
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
/// let xml_string = json_to_xml_with_root(&json_string, "People").unwrap();
///
/// println!("{}", xml_string);
/// ```
///
/// ## Expected Output (XML):
///
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// <!--Generated with json_to_xml 0.1.7-->
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
/// - `json_string`: The input JSON string to be converted into XML.
/// - `root`: The name for the root element of the XML. It will become the root element of the XML if the JSON contains top-level @ attributes.
///
/// ## Returns:
/// A `Result` which is either a `String` containing the XML representation of the input JSON, or a `ConversionError` if parsing or conversion fails.
///
/// ## Notes:
/// - This function works recursively to handle nested structures and arrays.
/// - JSON keys starting with `@` are treated as attributes for the parent XML element.
/// - All XML element tags are automatically capitalized.
/// - Empty JSON objects (`{}`) are converted into self-closing tags (e.g., `<Tag/>`).
/// - Empty JSON arrays (`[]`) are converted into an empty element (e.g., `<Tag></Tag>`).
/// - `null` values in JSON are converted into a self-closing `<None/>` tag.
pub fn json_to_xml_with_root(json_string: &str, root: &str) -> Result<String, ConversionError> {
    let json_value: Value = from_str(&json_string)?;
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    write_declaration(&mut writer, "1.0", Some("UTF-8"))?;
    
    let mut version = get_dependency_version("Cargo.toml")?;
    version = format!("Generated with json_to_xml {}", version);
    write_comment(&mut writer, &version)?;
    
    create_xml_element(&json_value, &mut writer, root)?;

    if has_top_level_attributes(&json_value) {
        write_end_tag(&mut writer, &BytesEnd::new(root))?;
    }

    Ok(String::from_utf8(writer.into_inner().into_inner())?)
}

/// Helper function to get json_to_xml version from the Cargo.toml file
pub fn get_dependency_version(file_path: &str) -> Result<String, ConversionError> {
    let content = fs::read_to_string(file_path)?;
    let toml: Value = toml_from_str(&content)?;

    if let Some(deps) = toml.get("dependencies") {
        if let Some(dep) = deps.get("json_to_xml") {
            if dep.is_object() {
                if let Some(version) = dep.get("version").and_then(|v| v.as_str()) {
                    return Ok(version.to_string());
                }
            } else if let Some(version) = dep.as_str() {
                return Ok(version.to_string());
            }
        }
    }

    // If not found in dependencies, try the package section
    if let Some(pkg) = toml.get("package") {
        if let Some(version) = pkg.get("version").and_then(|v| v.as_str()) {
            return Ok(version.to_string());
        }
    }

    Err(ConversionError::Toml(toml_from_str::<Value>("").unwrap_err()))
}

fn create_xml_element(
    json_data: &Value, 
    writer: &mut Writer<Cursor<Vec<u8>>>, 
    parent_tag: &str
) -> Result<(), ConversionError> {
    match json_data {
        Value::Object(map) => {
            handle_object(writer, map, parent_tag)?;
        },
        Value::Array(arr) => {
            handle_array(writer, arr, parent_tag)?;
        },
        Value::String(s) => {
            write_content(writer, s)?;
        },
        Value::Number(num) => {
            write_content(writer, &num.to_string())?;
        }
        Value::Bool(b) => {
            write_content(writer, &b.to_string())?;
        }
        Value::Null => {
            write_empty_tag(writer, &BytesStart::new("None"))?;
        }
    }
    Ok(())
}

fn handle_object(
    writer: &mut Writer<Cursor<Vec<u8>>>, 
    map: &Map<String, Value>, 
    parent_tag: &str
) -> Result<(), ConversionError> {
    let parent_tag = capitalize_word(parent_tag);
    let mut element = BytesStart::new(parent_tag.clone());

    let attributes: HashMap<_, _> = map
        .iter()
        .filter(|(key, _)| key.starts_with("@"))
        .map(|(key, value)| (&key[1..], value))
        .collect();

    for (key, value) in &attributes {
        if let Some(value_str) = value.as_str() {
            element.push_attribute((*key, value_str));
        }
    }

    if !attributes.is_empty() {
        write_start_tag(writer, &element)?;
    }

    if let Some(text_content) = map.get("$text").and_then(|v| v.as_str()) {
        write_content(writer, text_content)?;
    }

    for (key, value) in map {
        let key_tag = capitalize_word(key);
        element = BytesStart::new(key_tag.clone());

        if value.as_object().map_or(false, |m| m.is_empty()) {
            write_empty_tag(writer, &element)?;
            continue;
        }

        if key.starts_with("@") || key == "$text" {
            continue;
        } else {
            if !(is_attribute_key(value) || is_array_with_attribute_key(value)) {
                write_start_tag(writer, &element)?;
            }

            create_xml_element(value, writer, key)?;
            
            if !value.is_array() {
                write_end_tag(writer, &BytesEnd::new(key_tag))?;
            }
        }
    }
    Ok(())
}

fn handle_array(
    writer: &mut Writer<Cursor<Vec<u8>>>, 
    arr: &Vec<Value>, 
    parent_tag: &str
) -> Result<(), ConversionError> {
    let mut parent_tag = capitalize_word(parent_tag);
    let original_tag = parent_tag.clone();
    let item_tag = parent_tag.clone() + "Item";
    let mixed_array = contains_objects_and_primitives(arr);

    if arr.is_empty() {
        write_end_tag(writer, &BytesEnd::new(&parent_tag))?;
        return Ok(());
    }

    for (i, value) in arr.iter().enumerate() {
        if value.is_object() {
            if i == 0 && mixed_array {
                parent_tag = item_tag.clone(); 
                write_start_tag(writer, &BytesStart::new(&parent_tag))?;
            }
            
            handle_object_array(writer, i, value, &parent_tag)?;
        } else {
            if parent_tag != item_tag { 
                parent_tag = item_tag.clone(); 
            }

            write_start_tag(writer, &BytesStart::new(&parent_tag))?;
            create_xml_element(value, writer, &original_tag)?;
            write_end_tag(writer, &BytesEnd::new(&parent_tag))?;

            if i == arr.len() - 1 {
                write_end_tag(writer, &BytesEnd::new(&original_tag))?;
            }
        }

        if i == arr.len() - 1 && value.is_object() && mixed_array {
            write_end_tag(writer, &BytesEnd::new(&original_tag))?;
        }
    }
    Ok(())
}

fn handle_object_array(
    writer: &mut Writer<Cursor<Vec<u8>>>, 
    index: usize, 
    value: &Value, 
    parent_tag: &str
) -> Result<(), ConversionError> {
    if let Some(obj) = value.as_object() {
        if !obj.is_empty() {
            let first_key = obj.keys().next().unwrap();
            
            if !first_key.starts_with("@") && index > 0 {
                write_start_tag(writer, &BytesStart::new(parent_tag))?;
            } 

            create_xml_element(value, writer, &parent_tag)?;
            write_end_tag(writer, &BytesEnd::new(parent_tag))?;
        }
    }
    Ok(())
}

fn contains_objects_and_primitives(arr: &Vec<Value>) -> bool {
    let mut contains_obj = false;
    let mut contains_primitive = false;

    for v in arr {
        if v.is_object() {
            contains_obj = true;
        } else {
            contains_primitive = true;
        }
        if contains_obj && contains_primitive {
            return true;
        }
    }

    false
}

fn has_top_level_attributes(json: &Value) -> bool {
    if let Value::Object(map) = json {
        map.keys().any(|key| key.starts_with('@'))
    } else {
        false
    }
}

fn is_attribute_key(value: &Value) -> bool {
    value.as_object().map_or(false, |m| m.keys().any(|key| key.starts_with("@")))
}

fn is_array_with_attribute_key(value: &Value) -> bool {
    value.as_array().and_then(|a| a.first()).map_or(false, |v| is_attribute_key(v))
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
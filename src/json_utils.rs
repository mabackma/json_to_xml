use crate::string_utils::lowercase_word;

use std::collections::HashMap;
use regex::Regex;

/// Converts the keys of a JSON object to lowercase and replaces @ with __.
/// 
/// ```rust
/// use schema_generator::json_utils::json_keys_to_lowercase;
/// let input = serde_json::json!({
///     "FooBar": {
///         "@KeyName": "value",
///         "NestedObject": {
///             "SomeKey": "value"
///         }
///     },
///     "AnotherField": "value"
/// });
/// 
/// let expected = serde_json::json!({
///     "fooBar": {
///         "__KeyName": "value",
///         "nestedObject": {
///             "someKey": "value"
///         }
///     },
///     "anotherField": "value"
/// });
/// 
/// let result = json_keys_to_lowercase(&input);
/// 
/// assert_eq!(result, expected);
/// ```
pub fn json_keys_to_lowercase(json: &serde_json::Value) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                new_map.insert(lowercase_word(&key), json_keys_to_lowercase(value));
            }
            serde_json::Value::Object(new_map)
        },
        serde_json::Value::Array(vec) => {
            let new_vec: Vec<serde_json::Value> = vec.iter().map(|v| json_keys_to_lowercase(v)).collect();
            serde_json::Value::Array(new_vec)
        },
        _ => json.clone(),
    }
}

/// Creates a map of namespace prefixes from the root element of the JSON data.
/// The keys are the last segment of the namespace and the values are the prefixes.
/// 
/// (e.g. "__xmlns:co": "http://standardit.tapio.fi/schemas/forestData/common" -> "common": "co").
pub fn extract_prefixes(json_data: &serde_json::Value) -> HashMap<String, String> {
    let mut prefixes: HashMap<String, String> = HashMap::new();

    match json_data {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if key.starts_with("__xmlns:") {
                    let prefix = key.split(':').last().unwrap().to_string();
                    let value_string = value.as_str().unwrap().to_string();

                    // Remove the date from value string (e.g. /2021/05/01)
                    let re = Regex::new(r"/\d{4}/\d{2}/\d{2}").unwrap();
                    let namespace = re.replace(&value_string, "").to_string();

                    // Extract the last segment of the namespace
                    let last_segment = namespace.split('/').last().unwrap().to_string();
                    let formatted_namespace = lowercase_word(&last_segment);

                    prefixes.insert(formatted_namespace, prefix);
                }
            }
        },
        _ => {}
    }

    prefixes
}

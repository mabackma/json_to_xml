use crate::string_utils::lowercase_word;

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

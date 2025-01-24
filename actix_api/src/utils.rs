use serde_json::Value;

/// Finds a name within a JSON value based on a specific "value-name" key.
///
/// # Arguments
/// - `item`: A reference to a `serde_json::Value` containing the JSON data to search.
/// - `value`: A `String` specifying the target "value-name" to match.
///
/// # Returns
/// - A `String` representing the associated name if found; otherwise, "UNKNOWN".
///
/// # Details
/// - The function searches for a "name" array in the given `item`. Each item in the array is expected
///   to have a "value-name" key. If the "value-name" matches the provided `value`, the corresponding
///   "value" field is returned as the name.
pub fn find_name(item: &Value, value: String) -> String {
    let mut name = String::from("UNKNOWN");

    if let Some(name_section) = item.get("name").and_then(Value::as_array) {
        for name_item in name_section {
            if let Some(value_name) = name_item.get("value-name").and_then(Value::as_str) {
                if value_name == value {
                    let possible_name = name_item
                        .get("value")
                        .unwrap_or(&Value::default())
                        .to_string();
                    name = possible_name;
                }
            }
        }
    }
    name
}


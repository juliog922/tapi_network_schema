use actix_web::{Error, error};
use serde_json::{Value, Map};

/// Retrieves a value from a JSON object using either a JSON Pointer or a direct key.
///
/// # Arguments
///
/// * `pointer` - A boolean indicating if the path is a JSON Pointer (true) or a direct key (false).
/// * `json` - A reference to the JSON object from which to retrieve the value.
/// * `path` - The path or key to use for retrieval.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the value at the specified path or an error if the path is not found.
pub fn matching(pointer: bool, json: &Value, path: &'static str) -> Result<Value, Error> {
    if pointer {
        match json.pointer(path) {
            Some(value) => Ok(value.clone()),
            None => Err(error::ErrorNotFound(path)),
        }
    } else {
        match json.get(path) {
            Some(value) => Ok(value.clone()),
            None => Err(error::ErrorNotFound(path)),
        }
    }
}

/// Converts a `Value` to a list of `Value`s.
///
/// # Arguments
///
/// * `value` - The JSON `Value` to convert to a list.
///
/// # Returns
///
/// * `Result<Vec<Value>, Error>` - Returns a vector of `Value`s if conversion is successful, otherwise an error.
pub fn to_list(value: Value) -> Result<Vec<Value>, Error> {
    match value.as_array() {
        Some(list) => Ok(list.clone()),
        None => Err(error::ErrorUnprocessableEntity("Cannot convert to list")),
    }
}

/// Filters a JSON object to include only specified keys.
///
/// # Arguments
///
/// * `data` - The JSON object to filter.
/// * `keys` - A slice of keys to retain in the filtered JSON object.
///
/// # Returns
///
/// * `Value` - A new JSON object containing only the specified keys.
pub fn filter_keys(data: &Value, keys: &[&str]) -> Value {
    let mut filtered_data = Map::new();

    if let Value::Object(map) = data {
        for &key in keys {
            if let Some(value) = map.get(key) {
                filtered_data.insert(key.to_string(), value.clone());
            }
        }
    }

    Value::Object(filtered_data)
}

/// Searches for a value in a JSON object and returns the parent value at a specific level.
///
/// # Arguments
///
/// * `json` - The JSON object to search within.
/// * `target` - The target value to search for.
/// * `levels_up` - The number of levels to go up from the found value to get the parent value.
/// * `parent_key` - The key to match in the parent value.
/// * `current_path` - A mutable reference to the current path being traversed (used internally).
/// * `parent_values` - A mutable reference to the list of parent values (used internally).
///
/// # Returns
///
/// * `Option<Value>` - The parent value at the specified level if found, otherwise `None`.
fn search(json: &Value, target: &Value, levels_up: usize, parent_key: &str, current_path: &mut Vec<String>, parent_values: &mut Vec<Value>) -> Option<Value> {
    match json {
        Value::Object(map) => {
            parent_values.push(json.clone());
            for (key, value) in map {
                current_path.push(key.clone());

                if let Some(_) = search(value, target, levels_up, parent_key, current_path, parent_values) {
                    if current_path.len() >= levels_up {
                        let parent_index = parent_values.len() - levels_up - 1;
                        if current_path[parent_index] == parent_key {
                            return Some(parent_values[parent_index].clone());
                        }
                    }
                }

                current_path.pop();
            }
            parent_values.pop();
        }
        Value::Array(array) => {
            for item in array {
                if let Some(found_value) = search(item, target, levels_up, parent_key, current_path, parent_values) {
                    return Some(found_value);
                }
            }
        }
        Value::String(s) => {
            if let Value::String(target_string) = target {
                if s == target_string {
                    return Some(json.clone());
                }
            }
        }
        Value::Number(n) => {
            if let Value::Number(target_number) = target {
                if n == target_number {
                    return Some(json.clone());
                }
            }
        }
        _ => {}
    }
    None
}

/// Finds a value in a JSON object and retrieves the parent value at a specified level.
///
/// # Arguments
///
/// * `json` - The JSON object to search within.
/// * `target` - The target value to search for.
/// * `levels_up` - The number of levels to go up from the found value to get the parent value.
/// * `parent_key` - The key to match in the parent value.
///
/// # Returns
///
/// * `Result<Value, Error>` - Returns the parent value at the specified level if found, otherwise an error.
pub fn find_value_with_parent_value(json: &Value, target: &Value, levels_up: usize, parent_key: &str) -> Result<Value, Error> {
    let mut current_path = Vec::new();
    let mut parent_values = Vec::new();
    match search(json, target, levels_up, parent_key, &mut current_path, &mut parent_values) {
        Some(found_it) => Ok(found_it),
        None => Err(error::ErrorNotFound("Cannot find value")),
    }
}

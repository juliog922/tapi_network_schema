use actix_web::{Error, error};
use serde_json::{Value, Map, from_str};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;

pub fn find_name(item: &Value, value: String) -> String {
    let mut name = String::default();

        if let Some(name_section) = item.get("name").and_then(Value::as_array) {
            for name_item in name_section {

                if let Some(value_name) =  name_item.get("value-name").and_then(Value::as_str) {
                    if value_name == &value {
                        let possible_name = name_item.get("value").unwrap_or(&Value::default()).to_string();
                        name = possible_name;
                    }
                }
            }
        }
    name
}


fn get_json_from_file(file_name: &str) -> Result<Value, Error> {
    let current_dir = env::current_dir()?;

    // Definir la ruta relativa
    let relative_path = Path::new(file_name);

    // Construir la ruta completa
    let file_path = current_dir.join(relative_path);

    // Abre el archivo
    let mut file = File::open(&file_path)?;

    // Lee el contenido del archivo en una cadena
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;

    // Convierte la cadena JSON en un valor `serde_json::Value`
    let v: Value = from_str(&json_str)?;
    Ok(v)
}

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

/// Searches for all target values in a JSON object, checking if their parent key
/// matches the specified key `levels_up` levels above.
///
/// # Arguments
///
/// * `json` - The JSON object to search within.
/// * `target` - The target value to search for.
/// * `levels_up` - The number of levels to go up from the found value to get the parent value.
/// * `parent_key` - The key to match in the parent value.
/// * `current_path` - A mutable reference to the current path being traversed (used internally).
/// * `parent_values` - A mutable reference to the list of parent values (used internally).
/// * `results` - A mutable reference to the vector that collects all matching parent values.
///
/// # Returns
///
/// * `()` - Results are stored in the `results` vector.
fn search_all(
    json: &Value,
    target: &Value,
    levels_up: usize,
    parent_key: &str,
    current_path: &mut Vec<String>,
    parent_values: &mut Vec<Value>,
    results: &mut Vec<Value>,
) {
    match json {
        Value::Object(map) => {
            parent_values.push(json.clone());
            for (key, value) in map {
                current_path.push(key.clone());

                if value == target {
                    // Ensure there's enough levels to look up
                    if parent_values.len() > levels_up {
                        let parent_index = parent_values.len() - levels_up - 1;
                        if let Some(parent_object) = parent_values[parent_index].as_object() {
                            if parent_object.contains_key(parent_key) {
                                results.push(parent_values[parent_index].clone());
                            }
                        }
                    }
                }

                // Recursively search within child elements
                search_all(value, target, levels_up, parent_key, current_path, parent_values, results);

                current_path.pop();
            }
            parent_values.pop();
        }
        Value::Array(array) => {
            for item in array {
                search_all(item, target, levels_up, parent_key, current_path, parent_values, results);
            }
        }
        _ => {}  // No need to handle scalar values directly in search_all
    }
}

/// Finds all parent values that match the target value `levels_up` levels above in a large JSON.
///
/// # Arguments
///
/// * `json` - The JSON object to search within.
/// * `target` - The target value to search for.
/// * `levels_up` - The number of levels to go up from the found value to get the parent key.
/// * `parent_key` - The key to match in the parent value.
///
/// # Returns
///
/// * `Vec<Value>` - A vector of matching parent values.
pub fn find_all_values_with_parent_value(
    json: &Value,
    target: &Value,
    levels_up: usize,
    parent_key: &str,
) -> Vec<Value> {
    let mut current_path = Vec::new();
    let mut parent_values = Vec::new();
    let mut results = Vec::new();

    // Call the search function
    search_all(json, target, levels_up, parent_key, &mut current_path, &mut parent_values, &mut results);

    results
}
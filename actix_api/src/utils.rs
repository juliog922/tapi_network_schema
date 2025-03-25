use crate::AppError;

use std::str::FromStr;
use std::process::Command;

use minidom::Element;
use regex::Regex;
use serde_json::{Map, Value};

/// This function uses the system's `ping` command.
/// - On Windows it uses `-n 1` to send 1 echo request.
/// - On Unix‑like systems it uses `-c 1` to send 1 echo request.
/// 
/// # Arguments
/// - `ip`: A &str that represent an device ip
/// 
/// # Returns
/// `true` if the IP is reachable, otherwise `false`.
pub fn is_reachable(ip: &str) -> bool {
    // Build the command arguments depending on the OS.
    let args = if cfg!(target_os = "windows") {
        vec!["-n", "1", ip]
    } else {
        vec!["-c", "1", ip]
    };

    // Execute the command.
    match Command::new("ping").args(&args).output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

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

/// Ensures that the XML contains a namespace declaration. If none exists, a default namespace is added.
///
/// # Arguments
/// - `xml`: A string slice containing the XML data.
///
/// # Returns
/// - A `String` with the updated XML including a default namespace if necessary.
fn ensure_namespace(xml: &str) -> String {
    if xml.contains("xmlns=") {
        return xml.to_owned();
    }

    let re = Regex::new(r#"(?s)^(.*?<\s*([\w:]+)([^>]*))>"#).unwrap();
    if let Some(caps) = re.captures(xml) {
        // caps[1]: all just after of '>'
        // caps[2]: element name
        // caps[3]: attributes already on it
        let prefix = &caps[1];
        let tag_name = &caps[2];
        let rest = &caps[3];

        let new_tag = format!("<{} xmlns=\"default\"{}>", tag_name, rest);

        return xml.replacen(&format!("{}>", prefix), &new_tag, 1);
    }

    xml.to_owned()
}

/// Recursively converts an XML node into a JSON value.
///
/// # Arguments
/// - `el`: A reference to an `Element` representing the XML node.
///
/// # Returns
/// - An `Option<Value>` containing the converted JSON object.
fn convert_node(el: &Element) -> Option<Value> {
    let binding = el.text();
    let text = binding.trim();
    let mut obj = Map::new();

    for (attr, value) in el.attrs() {
        let key = format!("{}", attr);
        obj.insert(key, Value::String(value.to_owned()));
    }

    if !text.is_empty() {
        obj.insert(
            "text".to_owned(),
            Value::String(text.to_owned()),
        );
    }

    for child in el.children() {
        let child_name = child.name().to_string();
        if let Some(child_value) = convert_node(child) {
            if let Some(existing) = obj.get_mut(&child_name) {
                if let Value::Array(arr) = existing {
                    arr.push(child_value);
                } else {
                    let old_value = existing.take();
                    *existing = Value::Array(vec![old_value, child_value]);
                }
            } else {
                obj.insert(child_name, child_value);
            }
        }
    }

    Some(Value::Object(obj))
}

/// Converts the XML root into a JSON map where the root element's name is the key.
///
/// # Arguments
/// - `el`: A reference to an `Element` representing the XML root.
///
/// # Returns
/// - A `Value` containing the structured JSON representation of the XML.
fn xml_to_map(el: &Element) -> Value {
    let mut map = Map::new();
    map.insert(el.name().to_string(), convert_node(el).unwrap_or(Value::Null));
    Value::Object(map)
}

/// Converts an XML string into a `serde_json::Value` using the given configuration.
///
/// # Arguments
/// - `xml`: A string slice containing the XML data.
///
/// # Returns
/// - A `Result<Value, AppError>` containing the JSON representation or an error if parsing fails.
pub fn xml_to_json(xml: &str) -> Result<Value, AppError> {
    let xml = ensure_namespace(xml);
    let root = Element::from_str(&xml)
        .map_err(|err| AppError::validation_error(err.to_string()))?;
    Ok(xml_to_map(&root))
}


pub fn find_token_key(value: &Value) -> Option<&str> {
    value.as_object()?.iter().find_map(|(key, val)| {
        if let Value::String(s) = val {
            if s.len() > 8 && !s.contains(' ') {
                return Some(key.as_str());
            }
        }
        None
    })
}

/// Module containing unit tests for the `find_name` function.
#[cfg(test)]
mod tests {
    use super::{
        find_name,
        xml_to_json,
        is_reachable,
        find_token_key
    };
    use serde_json::json;

    /// Tests the `find_name` function with valid and invalid inputs.
    ///
    /// # Scenarios
    /// 1. Valid key: Checks that `find_name` correctly retrieves the name
    ///    associated with the specified "value-name".
    /// 2. Invalid key: Ensures that `find_name` returns `"UNKNOWN"` when the
    ///    specified "value-name" does not exist.
    #[test]
    fn test_find_name() {
        let json_data = json!({
            "name": [
                { "value-name": "SERVICE_NAME", "value": "My Service" },
                { "value-name": "OTHER_NAME", "value": "Other Service" }
            ]
        });
        let name = find_name(&json_data, "SERVICE_NAME".to_string());
        assert_eq!(name, "\"My Service\"");

        let unknown_name = find_name(&json_data, "NON_EXISTENT".to_string());
        assert_eq!(unknown_name, "UNKNOWN");
    }

    #[test]
    fn test_google_ping() {
        assert!(is_reachable("google.com"))
    }
    
    #[test]
    fn test_xml_to_json() {
        let xml = r#"<a attr1="1"><b><c attr2="001">some text</c></b></a>"#;
        let expected_json = json!({
            "a": {
                "attr1": "1",
                "b": {
                    "c": {
                        "attr2": "001",
                        "text": "some text"
                    }
                }
            }
        });

        let result = xml_to_json(xml).unwrap();
        assert_eq!(result, expected_json);
    }

    #[test]
    fn test_xml_to_json_empty_element() {
        let xml = r#"<root><empty /></root>"#;
        let expected_json = json!({
            "root": {
                "empty": {}
            }
        });

        let result = xml_to_json(xml).unwrap();
        assert_eq!(result, expected_json);
    }

    #[test]
    fn test_xml_to_json_multiple_children() {
        let xml = r#"<XRD xmlns='http://docs.oasis-open.org/ns/xri/xrd-1.0'>
        <Link rel='restconf' href='/restconf'/>
        </XRD>"#;
        let expected_json = json!(
            {
                "XRD" : {
                    "Link": {
                        "rel": "restconf",
                        "href": "/restconf"
                    }     
                }
            }   
        );

        let result = xml_to_json(xml).unwrap();
        assert_eq!(result, expected_json);
    }

    #[test]
    fn test_xml_to_json_attributes_only() {
        let xml = r#"<root><child>1</child><child>2</child></root>"#;
        let expected_json = json!({
            "root": {
                "child": [
                    { "text": "1" },
                    { "text": "2" }
                ]
            }
        });

        let result = xml_to_json(xml).unwrap();
        assert_eq!(result, expected_json);
    }

    #[test]
    fn test_find_token() {
        let json = serde_json::json!({
            "username": 123,
            "accessSession": false,
            "nested": {"key": "value"},
            "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
            "empty_string": "",
            "small_string": "abc",
            "text": "invalid token with spaces"
        });

        assert_eq!(find_token_key(&json), Some("token"));
    }

    #[test]
    fn test_none_token() {
        let json = serde_json::json!({
            "username": 123,
            "accessSession": false,
            "nested": {"key": "value"},
            "empty_string": "",
            "small_string": "abc",
            "token": "invalid token with spaces"
        });

        assert_eq!(find_token_key(&json), None);
    }
}
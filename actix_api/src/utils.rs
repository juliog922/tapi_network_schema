use serde_json::Value;

pub fn find_name(item: &Value, value: String) -> String {
    let mut name = String::from("UNKNOWN");

    if let Some(name_section) = item.get("name").and_then(Value::as_array) {
        for name_item in name_section {
            if let Some(value_name) = name_item.get("value-name").and_then(Value::as_str) {
                if value_name == &value {
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

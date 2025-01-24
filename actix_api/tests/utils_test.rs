/// Module containing unit tests for the `find_name` function.
#[cfg(test)]
mod tests {
    use serde_json::json;
    use actix_api::utils::find_name;

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
}
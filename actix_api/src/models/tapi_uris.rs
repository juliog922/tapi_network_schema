#[derive(sqlx::FromRow, Debug)]
pub struct TapiUri {
    pub id: i32,
    pub uri: String,
    pub request_method: String,
    pub topic: String,
    pub dependency: Option<String>,
}

impl std::fmt::Display for TapiUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
            id: {},
            uri: {},
            request_method: {},
            topic: {},
            dependency: {:?}
        "#,
            self.id, self.uri, self.request_method, self.topic, self.dependency
        )
    }
}

use crate::{
    AppError,
    models::tapi_uris::TapiUri,
    handlers::database::{DatabaseHandler, SqlxBindValue}
};

impl TapiUri {
    pub async fn read_one_by_topic(database_handler: &DatabaseHandler, topic: impl Into<String>) -> Result<Self, AppError> {
        database_handler.fetch_one::<Self>(
            "SELECT * FROM tapi_uris WHERE topic = $1", 
            vec![SqlxBindValue::Str(topic.into()),]
        ).await
    }
    
    pub async fn read_all(database_handler: &DatabaseHandler) -> Result<Vec<Self>, AppError> {
        database_handler.fetch_all::<Self>(
            "SELECT * FROM tapi_uris WHERE NOT dependency = '?'", 
            vec![]
        ).await
    }
}
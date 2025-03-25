use crate::{
    AppError,
    models::user::{User, UserDB},
    handlers::database::{DatabaseHandler, SqlxBindValue} 
};

impl User {
    pub async fn read_one_by_id(database_handler: &DatabaseHandler, id: impl Into<String>) -> Result<UserDB, AppError> {
        database_handler.fetch_one::<UserDB>(
            r#"SELECT id FROM public.users WHERE id = $1"#, 
            vec![SqlxBindValue::Str(id.into()),]
        ).await
    }

    pub async fn read_all(database_handler: &DatabaseHandler) -> Result<Vec<UserDB>, AppError> {
        database_handler.fetch_all::<UserDB>(
            "SELECT * FROM public.users", 
            vec![]
        ).await
    }
}
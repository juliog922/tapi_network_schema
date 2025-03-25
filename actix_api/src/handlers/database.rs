use sqlx::{postgres::PgPoolOptions, FromRow};

use crate::AppError;

#[derive(Debug, Clone)]
pub struct DatabaseHandler {
    pub connection: sqlx::Pool<sqlx::Postgres>,
}

// 📌 Enum para permitir diferentes tipos en `bind()`
pub enum SqlxBindValue {
    Int(i32),
    Str(String),
    Json(serde_json::Value),
    Null
}

impl DatabaseHandler {
    pub async fn new(database_url: impl AsRef<str>) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_ref())
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        Ok(Self { connection: pool })
    }

    pub async fn fetch_one<T>(&self, query: &str, params: Vec<SqlxBindValue>) -> Result<T, AppError> 
        where T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin
        {
        let mut query_builder = sqlx::query_as::<_, T>(query);

        // 📌 Bind dinámico sin errores de vida útil
        for param in params {
            query_builder = match param {
                SqlxBindValue::Int(value) => query_builder.bind(value),
                SqlxBindValue::Str(value) => query_builder.bind(value),
                SqlxBindValue::Json(value) => query_builder.bind(sqlx::types::Json(value)),
                SqlxBindValue::Null => query_builder.bind(None::<String>)
            };
        }

        let result = query_builder
            .fetch_one(&self.connection)
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        Ok(result)
    }

    pub async fn fetch_all<T>(&self, query: &str, params: Vec<SqlxBindValue>) -> Result<Vec<T>, AppError> 
        where T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin
        {
        let mut query_builder = sqlx::query_as::<_, T>(query);

        // 📌 Bind dinámico sin errores de vida útil
        for param in params {
            query_builder = match param {
                SqlxBindValue::Int(value) => query_builder.bind(value),
                SqlxBindValue::Str(value) => query_builder.bind(value),
                SqlxBindValue::Json(value) => query_builder.bind(sqlx::types::Json(value)),
                SqlxBindValue::Null => query_builder.bind(None::<String>)
            };
        }

        let result = query_builder
            .fetch_all(&self.connection)
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        Ok(result)
    }
}
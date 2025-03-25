use crate::{
    AppError,
    models::devices::{Device, Auth, TokenAuth, BasicAuth},
    handlers::database::{DatabaseHandler, SqlxBindValue} 
};
use sqlx::{FromRow, postgres::PgRow, Row};

impl<'r> FromRow<'r, PgRow> for Device {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        // Obtener los campos básicos
        let ip: String = row.try_get("ip")?;
        let port: Option<i32> = row.try_get("port")?;
        let port = port.map(|p| p as i64);

        // Obtener los campos relacionados a la autenticación
        let auth_body: sqlx::types::Json<serde_json::Value> = row.try_get("auth_body")?;
        let auth_type: String = row.try_get("auth_type")?;
        let auth_uri: Option<String> = row.try_get("auth_uri")?;

        // Construir el enum Auth según el tipo de autenticación
        let auth = match auth_type.as_str() {
            "basic" => {
                // Se espera que auth_body contenga un objeto JSON con "username" y "password"
                let username = auth_body.0.get("username")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| sqlx::Error::ColumnDecode {
                        index: "auth_body.username".into(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Falta el campo username en auth_body",
                        )),
                    })?
                    .to_string();

                let password = auth_body.0.get("password")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| sqlx::Error::ColumnDecode {
                        index: "auth_body.password".into(),
                        source: Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Falta el campo password en auth_body",
                        )),
                    })?
                    .to_string();

                Auth::Basic(BasicAuth { username, password })
            }
            "token" => {
                // Se espera que auth_uri esté presente para autenticación de tipo token
                let auth_uri = auth_uri.ok_or_else(|| sqlx::Error::ColumnDecode {
                    index: "auth_uri".into(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Falta auth_uri para autenticación token",
                    )),
                })?;
                Auth::Token(TokenAuth {
                    auth_body: auth_body.0,
                    auth_uri,
                })
            }
            other => {
                return Err(sqlx::Error::ColumnDecode {
                    index: "auth_type".into(),
                    source: Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Tipo de autenticación desconocido: {}", other),
                    )),
                });
            }
        };

        Ok(Device { ip, port, auth })
    }
}

impl Device {

    /// Convierte la instancia de Device en un vector de SqlxBindValue para usar en bind dinámico.
    /// El orden de los valores es: ip, port, auth_body, auth_type, auth_uri.
    pub fn to_bind_values(&self) -> Vec<SqlxBindValue> {
        let mut binds = Vec::new();

        // ip: siempre se envía como String.
        binds.push(SqlxBindValue::Str(self.ip.clone()));

        // port: se transforma de Option<i64> a Option<i32> (o Null si es None).
        match self.port {
            Some(p) => binds.push(SqlxBindValue::Int(p as i32)),
            None => binds.push(SqlxBindValue::Null),
        }

        // Para auth, diferenciamos entre Basic y Token.
        match &self.auth {
            Auth::Basic(basic_auth) => {
                binds.push(SqlxBindValue::Str("basic".to_string()));
                // Creamos el JSON con username y password.
                let auth_body = serde_json::json!({
                    "username": basic_auth.username,
                    "password": basic_auth.password
                });
                binds.push(SqlxBindValue::Json(auth_body));
                // Para basic, no se utiliza auth_uri.
                binds.push(SqlxBindValue::Null);
            }
            Auth::Token(token_auth) => {
                // Para token, el auth_body se envía tal cual.
                binds.push(SqlxBindValue::Str("token".to_string()));
                binds.push(SqlxBindValue::Json(token_auth.auth_body.clone()));
                binds.push(SqlxBindValue::Str(token_auth.auth_uri.clone()));
            }
        }

        binds
    }

    pub async fn read_one_by_ip(database_handler: &DatabaseHandler, ip: impl Into<String>) -> Result<Self, AppError> {
        database_handler.fetch_one::<Self>(
            r#"SELECT ip, port, auth_type, auth_body, auth_uri FROM devices WHERE ip = $1"#, 
            vec![SqlxBindValue::Str(ip.into()),]
        ).await
    }

    pub async fn create_device(&self, database_handler: &DatabaseHandler) -> Result<String, AppError> {
        let bind_values = self.to_bind_values();
        let (ip,): (String,) = database_handler.fetch_one(
            r#"INSERT INTO devices (ip, port, auth_type, auth_body, auth_uri) VALUES ($1, $2, $3, $4, $5) RETURNING ip"#,
            bind_values,
        ).await?;
        Ok(ip)
    }

    pub async fn delete_device(database_handler: &DatabaseHandler, ip: impl Into<String>) -> Result<String, AppError> {
        let (ip,): (String,) = database_handler.fetch_one(
            r#"DELETE FROM devices WHERE ip = $1 RETURNING ip"#,
            vec![SqlxBindValue::Str(ip.into()),],
        ).await?;
        Ok(ip)
    }
}
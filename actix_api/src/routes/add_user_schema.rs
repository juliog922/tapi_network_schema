use crate::handlers::{
    database::DatabaseHandler,
    http::HttpHandler
};
use crate::models::user::User;
use crate::utils::is_reachable;

use actix_web::{error, post, web, Error, HttpResponse};
use serde_json::json;

#[post("/add_device")]
pub async fn add_device(
    database: web::Data<DatabaseHandler>,
    new_user: web::Json<User>,
) -> Result<HttpResponse, Error> {
    todo!()
}
use crate::handlers::{
    database::DatabaseHandler,
    http::HttpHandler
};
use crate::models::devices::{Device, Auth};
use crate::utils::is_reachable;

use actix_web::{error, post, web, Error, HttpResponse};
use serde_json::json;

#[post("/add_device")]
pub async fn add_device(
    database: web::Data<DatabaseHandler>,
    request_device: web::Json<Device>,
) -> Result<HttpResponse, Error> {

    if is_reachable(&request_device.ip) {
        println!("{} is reachable!", &request_device.ip);
        let ret_ip: String;
        match &request_device.auth {
            Auth::Basic(_) => {
                ret_ip = request_device.create_device(&database).await.map_err(|_| error::ErrorServiceUnavailable("Database disconnected"))?;
                println!("{} device saved!", &ret_ip);
            },
            Auth::Token(token_auth) => {
                match HttpHandler::get_token(&request_device.get_full_auth_url(), &token_auth.auth_body).await {
                    Ok(_) => {
                        ret_ip = request_device.create_device(&database).await.map_err(|_| error::ErrorServiceUnavailable("Database disconnected"))?;
                        println!("{} device saved!", &ret_ip);
                    },
                    Err(err) => {
                        return Err(error::ErrorUnprocessableEntity(format!("Device cannot be added. {}", err.to_string())));
                    }
                }
            }
        }
        
        Ok(HttpResponse::Ok().json(json!({"message": &format!("{} added successfully", &ret_ip)})))
    } else {
        println!("{} is not reachable.", &request_device.ip);
        Err(error::ErrorUnprocessableEntity("Device cannot be added. ip is not reacheable by ping method"))
    }

}
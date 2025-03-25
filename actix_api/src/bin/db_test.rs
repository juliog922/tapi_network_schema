use std::env;

use actix_api::{
    handlers::{
        database::DatabaseHandler, http::HttpHandler
    }, models::devices::{Auth, Device}, utils::is_reachable, AppError
};
use serde_json::{from_str, Value};

use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;



#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();
    let binding = env::var("CYPHER_KEY").unwrap();
    let cyhper_key: &[u8] = binding.as_bytes();

    let binding = env::var("SALT").unwrap();
    let salt: &[u8] = binding.as_bytes();

    let mut key = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(cyhper_key, salt, 100_000, &mut key)
        .expect("HMAC can be initialized with any key length");
    println!("{:?}", key);

    let database_url = env::var("DATABASE_URL").unwrap();
    let database_handler = DatabaseHandler::new(database_url).await?;

    let json_data = r#"
    {
        "ip": "10.95.87.21",
        "port": 18010,
        "auth": {
            "username": "tapi",
            "password": "Zte_2025"
        }
    }
    "#;

    // Convert the JSON string into a serde_json::Value
    let json_value: Value = from_str(&json_data).expect("Json test data cannot be transformed to Value type");

    // Create a `Device` instance from the JSON `Value`
    let device: Device = serde_json::from_value(json_value).expect("Device cannot be created");

    if is_reachable(&device.ip) {
        let ret_ip: String;
        match &device.auth {
            Auth::Basic(_) => {
                ret_ip = device.create_device(&database_handler).await?;
                println!("{} device saved!", &ret_ip);
            },
            Auth::Token(token_auth) => {
                match HttpHandler::get_token(&device.get_full_auth_url(), &token_auth.auth_body).await {
                    Ok(_) => {
                        ret_ip = device.create_device(&database_handler).await?;
                        println!("{} device saved!", &ret_ip);
                    },
                    Err(err) => {
                        return Err(AppError::RequestError(format!("Device cannot be added. {}", err.to_string())));
                    }
                }
            }
        }
    } else {
        return Err(AppError::RequestError(format!("{} is not reachable.", &device.ip)));
    }


    println!("This is your saved device: {:?}", Device::read_one_by_ip(&database_handler, &device.ip).await?);

    println!("Device {} deleted.", Device::delete_device(&database_handler, &device.ip).await?);

    Ok(())
}

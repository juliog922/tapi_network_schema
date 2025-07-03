use crate::models::devices::Device;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use serde_json::json;

#[post("/{id}/{service_uuid}")]
pub async fn home(
    request: HttpRequest,
    path: web::Path<(String, String)>,
    body: web::Json<Device>,
) -> Result<HttpResponse, Error> {
    let (id, service_uuid) = path.into_inner();

    // Extraer headers
    let user = request
        .headers()
        .get("user")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("none");
    let role = request
        .headers()
        .get("role")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("none");

    println!("👤 user: {}", user);
    println!("🔐 role: {}", role);
    println!("📦 body: {:?}", body);

    Ok(HttpResponse::Ok().json(json!({
        "id": &id,
        "service_uuid": &service_uuid,
        "device": &body,
        "user": user,
        "role": role
    })))
}

use actix_web::{get, web, Responder};

use crate::utils::api_response::ApiResponse;

#[get("/hello/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    let api_response = ApiResponse::new(
        200,
        true,
        format!("Hello {}!", name),
        Some(format!("Hello {}!", name)),
    );
    api_response
}

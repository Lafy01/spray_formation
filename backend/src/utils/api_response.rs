use std::fmt::Display;

use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status_code: u16,
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    // response_code: StatusCode
}

impl<T> ApiResponse<T> {
    pub fn new(status_code: u16, success: bool, message: String, data: Option<T>) -> Self{
        ApiResponse {
            status_code,
            success,
            message,
            data,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::build(StatusCode::from_u16(self.status_code).unwrap())
            .content_type("application/json")
            .body(body)
    }
}

impl<T: Serialize + std::fmt::Debug> Display for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "Status Code: {}, Success: {}, Message: {}, Data: {:?}", 
            self.status_code, self.success, self.message, self.data)
    }
}

impl<T: Serialize + std::fmt::Debug> ResponseError for ApiResponse<T> {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code).unwrap()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = match serde_json::to_string(&self) {
            Ok(body) => body,
            Err(_) => json!({
                "status_code": 500,
                "success": false,
                "message": "Internal Server Error",
                "data": null
            }).to_string(),
        };
        HttpResponse::build(StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .content_type("application/json")
            .body(body)
    }
    
}
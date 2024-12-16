use std::fmt;

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    status: u16,
    message: String,
}

impl ApiError {
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        let message = message.into();
        ApiError { status, message }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl From<chrono::ParseError> for ApiError {
    fn from(_: chrono::ParseError) -> Self {
        ApiError::new(400, "Invalid time format")
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).json(json!({ "message": self.message }))
    }
}

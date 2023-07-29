use axum::Json;
use axum::http::header;
use axum::{http::StatusCode, response::IntoResponse};
use axum::response::Response;
use serde_json::json;


#[derive(Debug)]
pub struct APIError {
    // Add fields that provide additional context about the error
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: Option<i32>
    // You can add more fields here if needed
}
impl IntoResponse for APIError{
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        (status_code,[(header::CONTENT_TYPE, "application/json")], Json(json!( {"StatusCode": self.status_code.as_u16(), "Message": self.message, "ErrorCode": self.error_code})) ).into_response()
    }
}

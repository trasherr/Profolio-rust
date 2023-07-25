use axum::http::StatusCode;


pub fn E404() -> StatusCode {
    StatusCode::NOT_FOUND
}

pub fn E500() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
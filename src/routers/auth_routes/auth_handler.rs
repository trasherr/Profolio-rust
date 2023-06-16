
use axum::{response::{IntoResponse}, http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn register() -> impl IntoResponse {
    let greeting = "fdsa";
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({ "message": hello + greeting })))
}
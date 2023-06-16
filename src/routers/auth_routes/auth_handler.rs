
use axum::{response::{IntoResponse}, http::StatusCode, Json, Extension};
use entity::user;
use sea_orm::{ActiveValue, Set, ActiveModelTrait, DatabaseConnection};
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn login() -> impl IntoResponse {
    let greeting = "fdsa";
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({ "message": hello + greeting })))
}

pub async fn register(Extension(conn): Extension<DatabaseConnection>) -> impl IntoResponse {

    let mut user = user::ActiveModel { 
        id: ActiveValue::Set(1),
        name: Set("Prakhar".to_owned()),
        uuid: Set(Uuid::new_v4()),
        email: Set("Prakhar@gmail.com".to_owned()) ,
        password: Set("Prakhar".to_owned()),
        phone: Set("Prakhar".to_owned()),
        phone_code: ActiveValue::set(91),
        experience: Set(0),
        ..Default::default()
    
    };
    let insert: user::Model = user.insert(&conn).await.unwrap();
    (StatusCode::OK, Json(json!({ "message": "er" })))
}
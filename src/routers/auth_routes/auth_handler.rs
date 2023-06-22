
use axum::{response::{IntoResponse}, http::StatusCode, Json, Extension};
use entity::user;
use sea_orm::{ActiveValue, Set, ActiveModelTrait, DatabaseConnection, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use digest::Digest;
use sha2::Sha256;
use crate::utils::jwt;

#[derive(Deserialize)]
pub struct CreateUser{
    email: String,
    password: String,
    name: String,
    phone: String,
    phone_code: i32
}


#[derive(Deserialize)]
pub struct LoginUser{
    email: String,
    password: String
}



fn create_hash<D>(msg: &str, mut hasher: D) -> String
where
    D: Digest,
    digest::Output<D>: std::fmt::LowerHex,
{
    hasher.update(msg);
    format!("{:x}", hasher.finalize())
}

pub async fn login(Extension(conn): Extension<DatabaseConnection>, Json(user_data): Json<LoginUser>) -> impl IntoResponse {

    let hashed = create_hash(&user_data.password, Sha256::default());


    let user = user::Entity::find().filter(
        Condition::all()
        .add(user::Column::Email.eq(&user_data.email))
        .add(user::Column::Password.eq(hashed))
    )
    .one(&conn)
    .await.unwrap();

    if user.is_none() {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "succeeded": false, "token": "", "errors": ["Invalid credentials"] })))
    }

    let token = jwt::encode_jwt(user.unwrap().email);

    (StatusCode::OK, Json(json!({ "succeeded": true, "token": token.unwrap(), "errors": [] })))
}

pub async fn register(Extension(conn): Extension<DatabaseConnection>, Json(user_data): Json<CreateUser>) -> impl IntoResponse {


    let hashed = create_hash(&user_data.password, Sha256::default());

    let user = user::ActiveModel { 
        name: Set(user_data.name),
        uuid: Set(Uuid::new_v4()),
        email: Set(user_data.email) ,
        password: Set(hashed),
        phone: Set(user_data.phone),
        phone_code: ActiveValue::set(user_data.phone_code),
        experience: Set(0),
        ..Default::default()
    
    };
    user.insert(&conn).await.unwrap();
    
    (StatusCode::OK, Json(json!({ "succeeded": true, "errors": [] })))
}
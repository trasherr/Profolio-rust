
use axum::{ http::StatusCode, Json, Extension};
use entity::user;
use sea_orm::{ActiveValue, Set, ActiveModelTrait, DatabaseConnection, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use digest::Digest;
use sha2::Sha256;
use crate::utils::{jwt, api_error::APIError};

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

#[derive(Serialize)]
pub struct AuthRes{
    token: String
}



fn create_hash<D>(msg: &str, mut hasher: D) -> String
where
    D: Digest,
    digest::Output<D>: std::fmt::LowerHex,
{
    hasher.update(msg);
    format!("{:x}", hasher.finalize())
}

pub async fn login(Extension(conn): Extension<DatabaseConnection>, Json(user_data): Json<LoginUser>) -> Result<Json<AuthRes>,APIError> {

    let hashed = create_hash(&user_data.password, Sha256::default());

    let user = user::Entity::find().filter(
        Condition::all()
        .add(user::Column::Email.eq(&user_data.email))
        .add(user::Column::Password.eq(hashed))
    )
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?
    .ok_or(APIError { error_code: None, message: "Invalid Credentials".to_owned(), status_code: StatusCode::UNAUTHORIZED})?;

    let token = jwt::encode_jwt(user.email);

    Ok(Json(AuthRes { token: token.unwrap() }))
}

pub async fn register(Extension(conn): Extension<DatabaseConnection>, Json(user_data): Json<CreateUser>) -> Result<Json<AuthRes>,APIError> {


    let hashed = create_hash(&user_data.password, Sha256::default());

    let user = user::ActiveModel { 
        name: Set(user_data.name),
        uuid: Set(Uuid::new_v4()),
        email: Set(user_data.email.clone()) ,
        password: Set(hashed),
        phone: Set(user_data.phone),
        phone_code: ActiveValue::set(user_data.phone_code),
        experience: Set(0),
        ..Default::default()
    
    };
    user.insert(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;
    
    let token = jwt::encode_jwt(user_data.email.clone());
    
    Ok(Json(AuthRes { token: token.unwrap() }))
}
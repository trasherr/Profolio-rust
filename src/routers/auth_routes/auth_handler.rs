
use axum::{ http::StatusCode, Json, Extension, extract::Path};
use chrono::{ Utc, Duration};
use entity::user;
use lettre::message::header::ContentType;
use rand::Rng;
use sea_orm::{ActiveValue, Set, ActiveModelTrait, DatabaseConnection, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use digest::Digest;
use sha2::Sha256;
use crate::utils::{jwt, api_error::APIError, mailer};

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


#[derive(Serialize,Deserialize)]
pub struct OtpVerify{
    email: String,
    code: String
}

#[derive(Serialize,Deserialize)]
pub struct ChangePassword{
    password: String,
    code: String,
    email: String
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

    let email = user_data.email.clone().to_lowercase();
    let hashed = create_hash(&user_data.password, Sha256::default());

    let user = user::Entity::find().filter(
        Condition::all()
        .add(user::Column::Email.eq(email))
        .add(user::Column::Password.eq(hashed))
    )
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?
    .ok_or(APIError { error_code: None, message: "Invalid Credentials".to_owned(), status_code: StatusCode::UNAUTHORIZED})?;

    let token = jwt::encode_jwt(user.email);

    Ok(Json(AuthRes { token: token.unwrap() }))
}

pub async fn forget_password(
    Extension(conn): Extension<DatabaseConnection>, 
    Path(user_email): Path<String>
) -> Result<(),APIError> {


    
    let email = user_email.to_lowercase();
    
    let user = user::Entity::find().filter(
        Condition::all()
        .add(user::Column::Email.eq(email))
    )
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?
    .ok_or(APIError { error_code: None, message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

     entity::otp::Entity::delete_many().filter(entity::otp::Column::UserId.eq(user.id)).exec(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?;

    let code = rand::thread_rng().gen_range(100_000..999_999);

    let current = Utc::now().naive_utc();
    let exp = current.clone() + Duration::minutes(30);
    entity::otp::ActiveModel{
        code: Set(code.to_string()),
        user_id: Set(user.id),
        created_at: Set(current),
        expire: Set(exp),
        ..Default::default()
    }.insert(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR })?;

    mailer::mailer("no-reply",&user.name,&user.email,"Forgot Password ",ContentType::TEXT_PLAIN,&format!("Otp for changing the password: {}",code)).await?;

    Ok(())
}

pub async fn check_otp(
    Extension(conn): Extension<DatabaseConnection>, 
    Json(otp_req): Json<OtpVerify>
) -> Result<(),APIError> {


    let otp = entity::otp::Entity::find()
    .filter(entity::otp::Column::Code.eq(otp_req.code))
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?
    .ok_or(APIError { error_code: None, message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    let user = entity::user::Entity::find_by_id(otp.user_id)
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?
    .ok_or(APIError { error_code: None, message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    match user.email == otp_req.email.to_lowercase() {
        true => Ok(()),
        false => Err(APIError { error_code: None, message: "Invalid Otp".to_owned(), status_code: StatusCode::UNAUTHORIZED})
    }
}

pub async fn change_password(
    Extension(conn): Extension<DatabaseConnection>, 
    Json(password_req): Json<ChangePassword>
) -> Result<(),APIError> {

    let user = entity::user::Entity::find()
    .filter(entity::user::Column::Email.eq(password_req.email.to_lowercase()))
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?
    .ok_or(APIError { error_code: None, message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    let otp = entity::otp::Entity::find()
    .filter(
        Condition::all()
        .add(entity::otp::Column::Code.eq(password_req.code))
        .add(entity::otp::Column::UserId.eq(user.id))
    )
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?
    .ok_or(APIError { error_code: None, message: "Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    let mut user_model: entity::user::ActiveModel = user.into();
    let password = create_hash(&password_req.password, Sha256::default());
    user_model.password = Set(password);

    user_model.update(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;   

    Ok(())
}

pub async fn register(Extension(conn): Extension<DatabaseConnection>, Json(user_data): Json<CreateUser>) -> Result<Json<AuthRes>,APIError> {

    let email = user_data.email.clone().to_lowercase();
    let check_user  = user::Entity::find()
    .filter(user::Column::Email.eq(email))
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    if check_user != None {
        return Err(APIError { error_code: None, message: "Account already exists!".to_owned(), status_code: StatusCode::CONFLICT});
    }


    let hashed = create_hash(&user_data.password, Sha256::default());

    let user = user::ActiveModel { 
        name: Set(user_data.name),
        uuid: Set(Uuid::new_v4()),
        email: Set(user_data.email.clone().to_lowercase()) ,
        password: Set(hashed),
        phone: Set(user_data.phone),
        phone_code: ActiveValue::set(user_data.phone_code),
        experience: Set(0),
        ..Default::default()
    
    };
    user.insert(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR })?;
    
    let token = jwt::encode_jwt(user_data.email.to_lowercase().clone());
    
    Ok(Json(AuthRes { token: token.unwrap() }))
}
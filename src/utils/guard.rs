use crate::utils::jwt::decode_jwt;
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::utils::api_error::APIError;
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, APIError> {

    let token = request.headers().typed_get::<Authorization<Bearer>>().ok_or(APIError { error_code: None, message: "Authentication failed".to_owned(), status_code: StatusCode::BAD_REQUEST})?.token().to_owned();
    let claims = decode_jwt(token).map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::UNAUTHORIZED})?.claims;
    let db = request.extensions().get::<DatabaseConnection>().ok_or(APIError { error_code: None, message: "Could not connect to database".to_owned(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(claims.email.to_lowercase()))
        .one(db)    
        .await
        .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    let Some(user) = user else {return Err(APIError { error_code: None, message: "Invalid token".to_owned(), status_code: StatusCode::UNAUTHORIZED})};

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
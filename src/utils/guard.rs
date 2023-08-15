use crate::utils::jwt::decode_jwt;
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use entity::user;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {

    let token = request.headers().typed_get::<Authorization<Bearer>>().ok_or(StatusCode::UNAUTHORIZED)?.token().to_owned();
    let claims = decode_jwt(token)?.claims;

    let db = request.extensions().get::<DatabaseConnection>().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user::Entity::find()
        .filter(user::Column::Email.eq(claims.email))
        .one(db)    
        .await
        .map_err(|_error| {
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let Some(user) = user else {return Err(StatusCode::UNAUTHORIZED)};

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
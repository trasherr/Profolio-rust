
use std::ptr::null;

use axum::{response::{IntoResponse}, http::StatusCode, Json, Extension, headers::{authorization::{self, Bearer}, Authorization}, TypedHeader};
use entity::user::{self, Model};
use sea_orm::{ActiveValue, Set, ActiveModelTrait, DatabaseConnection, ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::Deserialize;
use serde_json::json;
use crate::utils::jwt;


#[derive(Deserialize)]
pub struct LoginUser{
    email: String,
    password: String
}



pub async fn get_user(Extension(conn): Extension<DatabaseConnection>, authorization: TypedHeader<Authorization<Bearer>>, Extension(user_info): Extension<Model>) -> impl IntoResponse {

    return (StatusCode::OK,Json(json!({ "succeeded": true, "user":  { "email": user_info.email, "name": user_info.name } })));


    let token = authorization.token();

    let tk = jwt::decode_jwt(token.to_owned());

    let claims: jwt::Claims = tk.unwrap().claims;

    // let hashed = create_hash(&user_data.password, Sha256::default());


    let user = user::Entity::find().filter(
        Condition::all()
        .add(user::Column::Email.eq(claims.email.to_owned()))
    )
    .one(&conn)
    .await.unwrap();


    if let Some(user) = user{
        (StatusCode::OK, Json(json!({ "succeeded": true, "user":  { "email": user.email, "name": user.name } })))

    }
    else{
        return (StatusCode::UNAUTHORIZED, Json(json!({ "succeeded": false, "user": null  })))

    }

}


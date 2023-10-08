use axum::{Extension, extract::Path};
use chrono::Utc;
use hyper::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};
use entity::remove_user;
use crate::utils::api_error::APIError;




pub async fn em_user_get(
    Extension(conn): Extension<DatabaseConnection>, 
    Path(email): Path<String>,
) -> Result<(),APIError>{

    let data = entity::user::Entity::find()
    .filter(entity::user::Column::Email.eq(email))
    .one(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(APIError { error_code: None, message: "Resource Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    let rm = remove_user::ActiveModel{
        user_id: Set(data.id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };
    rm.insert(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;


    Ok(())

}
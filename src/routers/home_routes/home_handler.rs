
use axum::{Extension, Json, http::StatusCode };
use entity::technology;
use sea_orm::{EntityTrait, DatabaseConnection };

use crate::{models::tech_model::TechModel, utils::api_error::APIError};

pub async fn get_tech(Extension(conn): Extension<DatabaseConnection>) -> Result<Json<Vec<TechModel>>,APIError>{

    let techs: Vec<TechModel> = technology::Entity::find().all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?

    .into_iter()
    .map(|item| TechModel { 
        uuid: item.uuid, 
        title: item.title, 
        normalized_title: item.normalized_title
    }).collect();

    Ok(Json(techs))

}

use axum::{Extension, http::StatusCode, Json, response::IntoResponse};
use entity::technology;
use sea_orm::{EntityTrait, DatabaseConnection };

use crate::models::tech_model::TechModel;

pub async fn get_tech(Extension(conn): Extension<DatabaseConnection>) -> impl IntoResponse{
    let techs: Vec<TechModel> = technology::Entity::find().all(&conn).await.unwrap().into_iter()
    .map(|item| TechModel { 
        uuid: item.uuid, 
        title: item.title, 
        normalized_title: item.normalized_title
    }).collect();

    (StatusCode::OK,Json(techs))

}
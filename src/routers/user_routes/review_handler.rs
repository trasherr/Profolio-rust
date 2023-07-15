
use axum::{response::IntoResponse, http::StatusCode, Json, Extension, extract::Path};
use chrono::{DateTime, Utc};
use entity::{user::{self, Model}, review_slot};
use sea_orm::{ DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait, Condition };

use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::models::review_model::ReviewSoltModel;

#[derive(Serialize,Deserialize)]
pub struct CreateSlot{
    pub slot_time: DateTime<Utc>,
}


pub async fn get_caption_slots(
    Extension(conn): Extension<DatabaseConnection>, 
    Path(caption_id): Path<Uuid>
) -> impl IntoResponse{

    let cap = user::Entity::find().filter(user::Column::Uuid.eq(caption_id)).one(&conn).await.unwrap().unwrap();

    let slots: Vec<ReviewSoltModel> = review_slot::Entity::find().filter(review_slot::Column::CaptionId.eq(cap.id)).all(&conn).await.unwrap().into_iter().map(|item| ReviewSoltModel {
        id: item.id,uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id
    }).collect();

    (StatusCode::OK,Json(slots))
}

pub async fn get_slots(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>,
    // Path(caption_id): Path<Uuid>
) -> impl IntoResponse{

    // let cap = user::Entity::find().filter(user::Column::Uuid.eq(caption_id)).one(&conn).await.unwrap().unwrap();

    let slots: Vec<ReviewSoltModel> = review_slot::Entity::find().filter(
        Condition::any()
        .add(review_slot::Column::CaptionId.eq(user.id))
        .add(review_slot::Column::UserId.eq(user.id))
    ).all(&conn).await.unwrap().into_iter()
    .map(|item| ReviewSoltModel {
        id: item.id,uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id
    }).collect();

    (StatusCode::OK,Json(slots))
}


pub async fn book_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>,
    Path(slot_id): Path<(i32)>
) -> impl IntoResponse{

    let slot = review_slot::Entity::find_by_id(slot_id).one(&conn).await.unwrap();
    let mut update_slot: review_slot::ActiveModel = slot.unwrap().into();
    update_slot.user_id = Set(Some(user.id));
    let res = update_slot.update(&conn).await
    .map(|item| ReviewSoltModel { id: item.id, uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id }).unwrap();

    (StatusCode::OK,Json(res))


}

pub async fn create_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>,
    Json(slot_data): Json<CreateSlot>
) -> impl IntoResponse{


    let res = review_slot::ActiveModel { 
        caption_id: Set(user.id),
        slot_time: Set(slot_data.slot_time.naive_local()),
        uuid: Set(Uuid::new_v4()),
        ..Default::default()
    }.insert(&conn).await;

    (StatusCode::OK,Json(res.map(|item| ReviewSoltModel { id: item.id, uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id }).unwrap()))

}

pub async fn get_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>,
    Path(uuid): Path<Uuid>
) -> impl IntoResponse{

    let slot = review_slot::Entity::find().filter(review_slot::Column::Uuid.eq(uuid)).one(&conn).await.unwrap().map(|item| ReviewSoltModel {
        id: item.id, uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id
    }).unwrap();

    if slot.caption_id == user.id || slot.user_id == Some(user.id){

        return (StatusCode::OK,Json(json!(slot)))
    }

    (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorised User" })))
}


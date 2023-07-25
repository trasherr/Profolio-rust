
use axum::{response::IntoResponse, http::StatusCode, Json, Extension, extract::Path};
use chrono::{DateTime, Utc};
use entity::{user::{self, Model}, review_slot};
use sea_orm::{ DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait  };

use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::models::{review_model::ReviewSoltModel, user_model::UserMicroModel};

#[derive(Serialize,Deserialize)]
pub struct CreateSlot{
    slot_time: DateTime<Utc>,
}


#[derive(Serialize,Deserialize)]
pub struct ReviewPostMeet{
    ratting: i32,
    desc: String,
}


pub async fn get_caption_slots(
    Extension(conn): Extension<DatabaseConnection>, 
    Path(caption_id): Path<Uuid>
) -> impl IntoResponse{

    let cap = user::Entity::find().filter(user::Column::Uuid.eq(caption_id)).one(&conn).await.unwrap().unwrap();

    let slots: Vec<ReviewSoltModel> = review_slot::Entity::find().filter(review_slot::Column::CaptionId.eq(cap.id)).all(&conn).await.unwrap().into_iter().map(|item| ReviewSoltModel {
        id: item.id,uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id,  caption: None
    }).collect();

    (StatusCode::OK,Json(slots))
}

pub async fn get_review(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<Model>,
    // Path(caption_id): Path<Uuid>
) -> impl IntoResponse{

    // let cap = user::Entity::find().filter(user::Column::Uuid.eq(caption_id)).one(&conn).await.unwrap().unwrap();
    let slots: Vec<ReviewSoltModel>= review_slot::Entity::find().filter(review_slot::Column::UserId.eq(identity.id))
    .find_with_related(user::Entity)
    .all(&conn).await.unwrap()
    .into_iter()
    .map(|item| 
        {
            let temp: UserMicroModel = item.1.first()
            .map(|item2| UserMicroModel { name: item2.name.to_owned(), company: item2.company.to_owned(), ctc: item2.ctc, uuid: item2.uuid }).unwrap();

            ReviewSoltModel {
                id: item.0.id,
                uuid: item.0.uuid, 
                user_id: item.0.user_id,
                slot_time: item.0.slot_time, 
                caption_id: item.0.caption_id, 
                caption: Some(temp),
            }
        }

    ).collect();
    
    (StatusCode::OK,Json(json!(slots)))
}


pub async fn book_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<Model>,
    Path(uuid): Path<Uuid>
) -> impl IntoResponse{

    let slot = review_slot::Entity::find().filter(review_slot::Column::Uuid.eq(uuid)).one(&conn).await.unwrap();
    let mut update_slot: review_slot::ActiveModel = slot.unwrap().into();
    update_slot.user_id = Set(Some(identity.id));
    let res = update_slot.update(&conn).await
    .map(|item| ReviewSoltModel { 
        id: item.id, 
        uuid: item.uuid, 
        user_id: item.user_id, 
        slot_time: item.slot_time, 
        caption_id: item.caption_id,
        caption: None
    }).unwrap();

    (StatusCode::OK,Json(res))


}

pub async fn create_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<Model>,
    Json(slot_data): Json<CreateSlot>
) -> impl IntoResponse{


    let res = review_slot::ActiveModel { 
        caption_id: Set(identity.id),
        slot_time: Set(slot_data.slot_time.naive_local()),
        uuid: Set(Uuid::new_v4()),
        ..Default::default()
    }.insert(&conn).await;

    (
        StatusCode::OK, 
        Json(res.map(|item| ReviewSoltModel {
            id: item.id, 
            uuid: item.uuid, 
            user_id: item.user_id, 
            slot_time: item.slot_time, 
            caption_id: item.caption_id,
            caption: None
        }).unwrap())
    )

}


pub async fn get_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<Model>,
    Path(uuid): Path<Uuid>
) -> impl IntoResponse{

    let slot = review_slot::Entity::find().filter(review_slot::Column::Uuid.eq(uuid)).one(&conn).await.unwrap().map(|item| ReviewSoltModel {
        id: item.id, uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id, caption: None
    }).unwrap();

    if slot.caption_id == identity.id || slot.user_id == Some(identity.id){

        return (StatusCode::OK,Json(json!(slot)))
    }

    (StatusCode::UNAUTHORIZED, Json(json!({ "succeeded": false, "error":[ "Unauthorised User"] }
)))
}


pub async fn save_review(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<Model>,
    Path(uuid): Path<Uuid>,
    Json(review_res): Json<ReviewPostMeet>
) -> impl IntoResponse{

    let slot = review_slot::Entity::find().filter(review_slot::Column::Uuid.eq(uuid)).one(&conn).await.unwrap().map(|item| ReviewSoltModel {
        id: item.id, uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id, caption: None
    }).unwrap();

    if slot.caption_id != identity.id {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "succeeded": false, "error":[ "Unauthorised User"] })))
    }


    let mut usr: user::ActiveModel = identity.into();

    usr.total_rating = Set(usr.total_rating.unwrap() + review_res.ratting);
    usr.total_reviews = Set(usr.total_reviews.unwrap() + 1);

    usr.update(&conn).await.unwrap();

    return (StatusCode::OK,Json(json!({ "succeeded": true, "error":[ ] })))

}
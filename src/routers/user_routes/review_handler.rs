
use axum::{http::StatusCode, Json, Extension, extract::Path };
use chrono::{DateTime, Utc, serde::ts_milliseconds};
use entity::{user, review_slot, order};
use sea_orm::{ DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait, Condition, QueryOrder, QuerySelect };
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{models::{review_model::ReviewSoltModel, user_model::UserMicroModel}, utils::api_error::APIError};

#[derive(Serialize,Deserialize)]
pub struct CreateSlot{
    #[serde(with = "ts_milliseconds")]
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
) -> Result<Json<Vec<ReviewSoltModel>>, APIError>{

    let captain_model = user::Entity::find().filter(user::Column::Uuid.eq(caption_id)).one(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    if captain_model == None {
        return Err(APIError { error_code: None, message: "Resource Not Found".to_string(), status_code: StatusCode::NOT_FOUND});
    }
    let captain = captain_model.unwrap();

    let slots: Vec<ReviewSoltModel> = review_slot::Entity::find()
    .filter(review_slot::Column::CaptionId.eq(captain.id))
    .order_by_asc(review_slot::Column::SlotTime)
    .all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .into_iter().map(|item| ReviewSoltModel {
        id: item.id,uuid: item.uuid, user_id: item.user_id, slot_time: item.slot_time, caption_id: item.caption_id,  caption: None
    }).collect();

    Ok(Json(slots))
}

pub async fn get_review(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>,
    // Path(caption_id): Path<Uuid>
) -> Result<Json<Vec<ReviewSoltModel>>,APIError>{

    // let cap = user::Entity::find().filter(user::Column::Uuid.eq(caption_id)).one(&conn).await.unwrap().unwrap();
    let slots: Vec<ReviewSoltModel>= review_slot::Entity::find().filter(review_slot::Column::UserId.eq(identity.id))
    .find_with_related(user::Entity)
    .order_by_asc(review_slot::Column::SlotTime)
    .all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
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
    
    Ok(Json(slots))
}


pub async fn get_review_count(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>,
    Path(meeting_count): Path<u64>
) -> Result<Json<Vec<ReviewSoltModel>>,APIError>{

    // let cap = user::Entity::find().filter(user::Column::Uuid.eq(caption_id)).one(&conn).await.unwrap().unwrap();
    let slots: Vec<ReviewSoltModel>= review_slot::Entity::find().filter(review_slot::Column::UserId.eq(identity.id))
    .find_with_related(user::Entity)
    .order_by_asc(review_slot::Column::SlotTime)
    .limit(meeting_count)
    .all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
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
    
    Ok(Json(slots))
}


pub async fn book_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>,
    Path(order_id): Path<String>
) -> Result<Json<ReviewSoltModel>, APIError>{

    println!("{}",order_id);
    let order_entity = order::Entity::find()
    .filter(order::Column::OrderId.eq(order_id)).one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    if order_entity == None {
        return Err(APIError { error_code: None, message: "Resource Not Found".to_string(), status_code: StatusCode::NOT_FOUND});
    }

    let order = order_entity.unwrap();

    
    let slot_entity = review_slot::Entity::find_by_id(order.slot_id).one(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    if slot_entity == None {
        return Err(APIError { error_code: None, message: "Resource Not Found".to_string(), status_code: StatusCode::NOT_FOUND});
    }
    let slot = slot_entity.unwrap();

    let mut update_slot: review_slot::ActiveModel = slot.into();
    update_slot.user_id = Set(Some(identity.id));
    let res = update_slot.update(&conn).await
    .map(|item| ReviewSoltModel { 
        id: item.id, 
        uuid: item.uuid, 
        user_id: item.user_id, 
        slot_time: item.slot_time, 
        caption_id: item.caption_id,
        caption: None
    })
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    Ok(Json(res))


}

pub async fn create_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>,
    Json(slot_data): Json<CreateSlot>
) -> Result<Json<ReviewSoltModel>,APIError>{

    if !identity.is_caption {
        return Err(APIError{ error_code: None, status_code: StatusCode::FORBIDDEN, message: "Not enough permissions".to_string() });
    }

    let res = review_slot::ActiveModel { 
        caption_id: Set(identity.id),
        slot_time: Set(slot_data.slot_time.naive_local()),
        uuid: Set(Uuid::new_v4()),
        ..Default::default()
    }.insert(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    Ok(
        Json( ReviewSoltModel {
            id: res.id, 
            uuid: res.uuid, 
            user_id: res.user_id, 
            slot_time: res.slot_time, 
            caption_id: res.caption_id,
            caption: None
        })
    )

}


pub async fn get_slot(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>,
    Path(slot_uuid): Path<Uuid>
) -> Result<Json<ReviewSoltModel >,APIError>{

    let slot_model = review_slot::Entity::find().filter(
        Condition::all()
        .add(review_slot::Column::Uuid.eq(slot_uuid))
        .add(
            Condition::any()
            .add(review_slot::Column::CaptionId.eq(identity.id))
            .add(review_slot::Column::UserId.eq(identity.id))
            )
        )
        .order_by_asc(review_slot::Column::SlotTime)
        .one(&conn).await
        .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;
    

    if slot_model == None {
        return Err(APIError { error_code: None, message: "Resource Not Found".to_string(), status_code: StatusCode::NOT_FOUND});
    }
    let slot = slot_model.unwrap();

    let caption_entity = user::Entity::find_by_id(slot.caption_id)
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    if caption_entity == None {
        return Err(APIError { error_code: None, message: "Resource Not Found".to_string(), status_code: StatusCode::NOT_FOUND});
    }

    let caption = caption_entity.unwrap();

    return Ok(Json( 
        ReviewSoltModel {
            id: slot.id, 
            uuid: slot.uuid, 
            user_id: slot.user_id, 
            slot_time: slot.slot_time, 
            caption_id: slot.caption_id, 
            caption: Some(
                UserMicroModel{
                    name: caption.name,
                    company: caption.company,
                    ctc: caption.ctc,
                    uuid: caption.uuid,
                }
            )
        }
    ));

}


pub async fn save_review(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>,
    Path(slot_uuid): Path<Uuid>,
    Json(review_res): Json<ReviewPostMeet>
) -> Result<StatusCode,APIError>{

    let slot = review_slot::Entity::find()
    .filter(review_slot::Column::Uuid.eq(slot_uuid))
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR })?
    .map(|item| ReviewSoltModel {
        id: item.id, 
        uuid: item.uuid, 
        user_id: item.user_id, 
        slot_time: item.slot_time, 
        caption_id: item.caption_id, 
        caption: None
    }).unwrap();

    if slot.caption_id != identity.id {
        let error = APIError { error_code: None,
            message: "Unauthorised".to_string(),
            status_code: StatusCode::UNAUTHORIZED
        };
        return Err(error);
    }


    let mut usr: user::ActiveModel = identity.into();

    usr.total_rating = Set(usr.total_rating.unwrap() + review_res.ratting);
    usr.total_reviews = Set(usr.total_reviews.unwrap() + 1);

    usr.update(&conn).await.unwrap();

    return Ok(StatusCode::OK);

}
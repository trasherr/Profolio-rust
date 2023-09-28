use chrono::NaiveDateTime;
use hyper::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

use crate::utils::api_error::APIError;

use super::user_model::UserMicroModel;


#[derive(Serialize,Deserialize, Default)]
pub struct ReviewSoltModel{
    pub id: i32,
    pub user_id: Option<i32>,
    pub user: Option<String>,
    pub uuid: Uuid,
    pub caption: Option<UserMicroModel>,
    pub caption_id: i32,
    pub slot_time: NaiveDateTime,
}

#[derive(Serialize,Deserialize, Default)]
pub struct ReviewSoltBookedModel{
    pub user_slots: Vec<ReviewSoltModel>,
    pub booked_slots: Vec<ReviewSoltModel>,
}


impl From<Value> for ReviewSoltModel {
    fn from(p: Value) -> Self {

        let temp: ReviewSoltModel = serde_json::from_value(p).map_err(|_| APIError{ message: "Incorrect Data".to_owned(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: None }).unwrap();
        Self {
            id: temp.id,
            user_id: temp.user_id,
            user: temp.user,
            uuid: temp.uuid,
            caption: temp.caption,
            caption_id: temp.caption_id,
            slot_time: temp.slot_time,
            ..Default::default()
        }
    }
}


impl From<entity::review_slot::Model> for ReviewSoltModel {
    fn from(p: entity::review_slot::Model) -> Self {

        Self {
            id: p.id,
            user_id: p.user_id,
            uuid: p.uuid,
            caption_id: p.caption_id,
            slot_time: p.slot_time,
            ..Default::default()
        }
    }
}


impl From<(entity::review_slot::Model,Vec<entity::user::Model>)> for ReviewSoltModel {
    fn from(p: (entity::review_slot::Model,Vec<entity::user::Model>)) -> Self {

        let temp: UserMicroModel = p.1.first()
        .map(|item2| UserMicroModel { 
            name: item2.name.to_owned(), 
            company: item2.company.to_owned(), 
            ctc: item2.ctc, 
            uuid: item2.uuid,
            ..Default::default()
        }).unwrap();
        Self {
            id: p.0.id,
            user_id: p.0.user_id,
            uuid: p.0.uuid,
            caption_id: p.0.caption_id,
            slot_time: p.0.slot_time,
            caption: Some(temp),
            ..Default::default()
        }
    }
}
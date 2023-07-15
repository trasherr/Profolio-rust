use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct ReviewSoltModel{
    pub id: i32,
    pub user_id: Option<i32>,
    pub uuid: Uuid,
    pub caption_id: i32,
    pub slot_time: NaiveDateTime,
}
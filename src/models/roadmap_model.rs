use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::user_model::UserMicroModel;


#[derive(Serialize,Deserialize)]
pub struct RoadmapModel{
    pub created_at: NaiveDateTime,
    pub id: i32,
    pub levels: Vec<LevelModel>,
    pub modified_at: NaiveDateTime,
    pub target: i32,
    pub uuid: Uuid,

}


#[derive(Serialize,Deserialize,Clone)]
pub struct LevelModel{
    pub id: i32,
    pub level: i32,
    pub user: UserMicroModel,
    // pub user_uuid: Uuid
}
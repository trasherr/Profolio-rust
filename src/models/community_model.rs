use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::user_model::UserModel;


#[derive(Serialize,Deserialize)]
pub struct CommunityModel{
    pub uuid: Uuid,
    pub tech_id: i32,
    pub ctc_range: f64,
    pub title: String,
    pub description: Option<String>,
    pub members: Vec<UserModel>

}
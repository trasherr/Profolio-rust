use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct CommunityModel{
    pub uuid: Uuid,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>

}
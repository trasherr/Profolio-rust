use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize)]
pub struct TechModel{
    pub uuid: Uuid,
    pub title: String,
    pub normalized_title: String,
}
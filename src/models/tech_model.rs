use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize,Deserialize,Clone)]
pub struct TechModel{
    pub uuid: Uuid,
    pub title: String,
    pub normalized_title: String,
}
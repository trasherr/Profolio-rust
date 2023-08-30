use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::tech_model::TechModel;


#[derive(Deserialize, Serialize, Clone,Default)]
pub struct UserModel{
    pub id: i32,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub phone_code: i32,
    pub ctc: i32,
    pub profession: Option<String>,
    pub experience: i32,
    pub company: Option<String>,
    pub uuid: Uuid,
    pub is_caption: bool,
    pub is_caption_applied: bool,
    pub linkedin:Option<String>,
    pub github: Option<String>,
    pub others: Option<String>,
    pub tech: Option<Vec<TechModel>>
}

#[derive(Serialize,Deserialize,Clone,Default,Debug)]
pub struct UserMicroModel{
    pub name: String,
    pub company: Option<String>,
    pub profession: Option<String>,
    pub ctc: i32,
    pub uuid: Uuid,
    pub tech: Option<Vec<TechModel>>,
    pub others: Option<String>
    // pub user_uuid: Uuid
}

impl From<entity::user::Model> for UserMicroModel {
    fn from(p: entity::user::Model) -> Self {
      Self {
        name: p.name.clone(),
        company: p.company.clone(),
        profession: p.profession.clone(),
        ctc: p.ctc,
        uuid: p.uuid,
        others: p.others.clone(),
        ..Default::default()
        }
    }
}

impl From<entity::user::Model> for UserModel {
    fn from(p: entity::user::Model) -> Self {
      Self {
        name: p.name,
        company: p.company,
        ctc: p.ctc,
        uuid: p.uuid,
        others: p.others,
        id: p.id,
        email: p.email,
        phone: p.phone,
        phone_code: p.phone_code,
        profession: p.profession,
        experience: p.experience,
        is_caption: p.is_caption,
        is_caption_applied:p.is_caption_applied,
        linkedin: p.linkedin.clone(),
        github: p.linkedin.clone(),
        ..Default::default()
        }
    }
}
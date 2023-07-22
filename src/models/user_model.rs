use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Deserialize, Serialize)]
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
    pub linkedin:Option<String>,
    pub github: Option<String>,
    pub others: Option<String>
}

#[derive(Serialize,Deserialize,Clone)]
pub struct UserMicroModel{
    pub name: String,
    pub company: Option<String>,
    pub ctc: i32,
    pub uuid: Uuid,
    // pub user_uuid: Uuid
}

use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize)]
pub struct UserModel{
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone: String,
    pub phone_code: i32,
    pub ctc: Option<i32>,
    pub profession: Option<String>,
    pub experience: i32,
    pub company: Option<String>,

    pub uuid: String,
    pub linkedin:String,
    pub github: String,
    pub others: String
}

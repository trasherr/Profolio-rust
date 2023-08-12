use chrono::{NaiveDateTime, DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct OrderModel{
    pub id: i32,
    pub user_id: i32,
    pub slot_id: i32,
    pub order_id: String,
    pub currency: String,
    pub amount: i32,
    pub amount_paid: i32,
    pub amount_due: i32,
    pub receipt: String,
    pub offer_id: Option<String>,
    pub status: String,
    pub attempts: i16,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}


#[derive(Serialize)]
pub struct OrderApiReq{
    pub amount: i32,
    pub currency: String,
    pub receipt: String,
      
}

impl Into<String> for OrderApiReq {
    fn into(self) -> String {
        format!( r#" {{"amount": {}, "currency": "{}","receipt": "{}"}}"#,self.amount,self.currency,self.receipt)
    }
    
}

#[derive(Serialize,Deserialize)]
pub struct OrderApiRes{
    pub id: String,
    pub entity: String,
    pub amount: i32,
    pub amount_paid: i32,
    pub amount_due: i32,
    pub currency: String,
    pub receipt: String,
    pub offer_id: Option<String>,
    pub status: String,
    pub attempts: i16,
    pub created_at: i64,
}

#[derive(Serialize,Deserialize)]
pub struct OrderApiSignature{
   pub order_id: String,
   pub signature: String,
   pub payment_id: String
}

use axum::{response::{IntoResponse}, http::StatusCode, Json, Extension, headers::{authorization::Bearer, Authorization}, TypedHeader};
use entity::{user::{self, Model}, user_technology, technology};
use sea_orm::{ query::*, DatabaseConnection, ColumnTrait, Condition, EntityTrait, QueryFilter, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use crate::utils::jwt;

#[derive(Deserialize)]
pub struct UserSubDetails{
    ctc: Option<i32>,
    profession: Option<String>,
    experience: i32,
    company: Option<String>,
}



#[derive(Deserialize,Serialize,Debug)]
pub struct TechnologyResponse{
    uuid: Uuid,
    title: String,
    normalized_title: String,
}


pub async fn update(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>, 
    Json(user_data): Json<UserSubDetails>
) -> impl IntoResponse {

    let mut u: user::ActiveModel = user::Entity::find_by_id(user.id).one(&conn).await.unwrap().unwrap().into();

    u.company = Set(user_data.company);

    u.ctc = Set(user_data.ctc.unwrap_or(0));
  
    u.profession = Set(user_data.profession);
    u.experience = Set(user_data.experience);

    let u: user::Model = u.update(&conn).await.unwrap();
    (StatusCode::OK, Json(json!({
        "email": u.email, 
        "name": u.name, 
        "uuid": u.uuid,
        "ctc": u.ctc,
        "profession": u.profession,
        "experience": u.experience,
        "company": u.company
    } ))) 


}

pub async fn add_tech(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>, 
    Json(technologies): Json<Vec<Uuid>>
) -> impl IntoResponse {

    let techs = technology::Entity::find()
    .filter(technology::Column::Uuid.is_in(technologies))
    .all(&conn)
    .await.map_err(|_| { (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "succeeded": true, "errors": ["Failed to get technologies"] }))) }).unwrap();


    let mut user_techs: Vec<user_technology::ActiveModel> = [].to_vec();
    for (i, item) in techs.into_iter().enumerate() {
        let temp: user_technology::ActiveModel = user_technology::ActiveModel { 
            user_id: Set(user.id),
            technology_id: Set(item.id),
            score: Set(1.0),
            ..Default::default()
        };
        user_techs.push(temp);
    }

    let res= user_technology::Entity::insert_many(user_techs).exec(&conn).await
    .map_err(|_| { (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({ "succeeded": true, "errors": ["Failed to add technology"] }))) });

    (StatusCode::OK, Json(json!({ "succeeded": true, "errors": [] })))
}


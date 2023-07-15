// use axum::{Extension, response::IntoResponse, Json, http::StatusCode, extract::Path, Error};
// use entity::user::Model;
// use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Condition};
use serde::{Deserialize, Serialize};
// use entity::community;
// use uuid::Uuid;

// use crate::models::community_model::CommunityModel;



#[derive(Serialize,Deserialize)]
pub struct CreateCommunity{
    pub title: String,
    pub description: Option<String>

}

// pub async fn create(
//     Extension(conn): Extension<DatabaseConnection>, 
//     Extension(user): Extension<Model>,
//     Json(community): Json<CreateCommunity>
// ) -> impl IntoResponse{

//     let comm = community::ActiveModel {
//         title: Set(community.title),
//         description: Set(community.description),
//         ..Default::default()
//     }.insert(&conn).await
//     .map_err(|_| { (StatusCode::INTERNAL_SERVER_ERROR) }).unwrap();

//     (StatusCode::OK,Json( CommunityModel { uuid: comm.uuid, title: comm.title, description: comm.description, user_id: comm.user_id } ))

// }


// pub async fn update(
//     Path(uuid): Path<Uuid>,
//     Extension(conn): Extension<DatabaseConnection>, 
//     Extension(user): Extension<Model>,
//     Json(community): Json<CreateCommunity>
// ) -> Result<(),StatusCode>{

//     let mut comm: community::ActiveModel = community::Entity::find()
//     .filter(
//         Condition::all()
//         .add(community::Column::Uuid.eq(uuid))
//         .add(community::Column::UserId.eq(user.id)) 
//     ).one(&conn).await.unwrap().unwrap().into();

//     comm.title = Set(community.title);
//     comm.description = Set(community.description);
    
//     let res = comm.update(&conn).await.unwrap();

//     Ok(())

// }
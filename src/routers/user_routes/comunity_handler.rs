// use entity::{community_user, user};
// use axum::{Extension, response::IntoResponse, Json, http::StatusCode, extract::Path, Error};
// use entity::user::Model;
// use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Condition};
// use serde::{Deserialize, Serialize};
// use entity::community;
// use uuid::Uuid;

// use crate::models::{community_model::CommunityModel, user_model::{self, UserModel}};



// #[derive(Serialize,Deserialize)]
// pub struct CreateCommunity{
//     pub title: String,
//     pub description: Option<String>

// }

// pub async fn community(
//     Extension(conn): Extension<DatabaseConnection>, 
//     Extension(user): Extension<Model>
// ) -> impl IntoResponse{

//     let comm_user = community_user::Entity::find().filter(user::Column::Id.eq(user.id)).one(&conn).await.unwrap().unwrap();

//     let comm = community::Entity::find().filter(community::Column::Id.eq(comm_user.community_id)).one(&conn).await.unwrap().unwrap();
    
//     let users: Vec<UserModel> = community_user::Entity::find().filter(community_user::Column::CommunityId.eq(comm.id)).find_with_related(user::Entity).all(&conn).await.unwrap()
//     .into_iter()
//     .map(|item| {

//         let data = item.1.first().unwrap();

//         return UserModel { 
//             id: data.id,
//             name:  data.name.clone(),
//             email:  data.email.clone(),
//             phone:  data.phone.clone(),
//             phone_code:  data.phone_code.clone(),
//             ctc:  data.ctc.clone(),
//             profession:  data.profession.clone(),
//             experience:  data.experience,
//             company:  data.company.clone(),

//             uuid:  data.uuid,
//             linkedin:  data.linkedin.clone(),
//             github:  data.github.clone(),
//             others:  data.others.clone()
//         }
//     })
//     .collect();
    
//     let res = CommunityModel { 
//         uuid: comm.uuid, 
//         tech_id: 0, 
//         ctc_range: comm.ctc_range, 
//         title: comm.title, 
//         description: comm.description, 
//         members: users
//     };

//     (StatusCode::OK,Json(res))

// }
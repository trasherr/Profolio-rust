use axum::{Extension, Json, http::StatusCode };
use chrono::Utc;
use entity::{user, leagues, roadmap, roadmap_user};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Condition, QueryOrder, Set, ActiveModelTrait };
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::Rng;
use crate::{models::roadmap_model::{RoadmapModel, LevelModel }, utils::api_error::APIError};
use crate::models::user_model::UserMicroModel;

#[derive(Serialize,Deserialize)]
pub struct RoadmapDetails{
   target_uuid: Uuid
}


pub async fn roadmap_post(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<user::Model>,
    Json(data): Json<RoadmapDetails>

)-> Result<StatusCode,APIError>{

    let target = user::Entity::find()
    .filter(
        Condition::all()
        .add(user::Column::Uuid.eq(data.target_uuid))
        .add(user::Column::IsCaption.eq(true))
       
    )
    .one(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(APIError { error_code: None, message: "Resource Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    let league = leagues::Entity::find().filter(
        Condition::all()
        .add(leagues::Column::CtcLower.lt(target.ctc))
        .add(leagues::Column::CtcLower.gt(user.ctc))       
    ).order_by_asc(leagues::Column::CtcLower).all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;


    //create path

    let roadmap = roadmap::ActiveModel {
        uuid: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        target_id: Set(target.id),
        created_at: Set(Utc::now().naive_utc()),
        modified_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }.insert(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    let mut current_level: i8 = 1;

    for item in league {


        let captions_all = user::Entity::find().filter(
            Condition::all()
            .add(user::Column::Ctc.gte(item.ctc_lower))
            .add(user::Column::Ctc.lte(item.ctc_lower))
            .add(user::Column::IsCaption.eq(true))
        ).order_by_asc(user::Column::Ctc).all(&conn).await
        .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;


        let count = captions_all.len();

        let mut required_captains: Vec<user::Model> = vec![];
        

        for _ in 1..5{
            let cap_ids = rand::thread_rng().gen_range(0..count);
            if required_captains.contains(&captions_all[cap_ids]) {
                continue;
            }
            required_captains.push(captions_all[cap_ids].clone());

        };
        if required_captains.len() == 0 as usize {
            continue;
        }
        // add to path
        for _item in required_captains{

            roadmap_user::ActiveModel{
                user_id: Set(_item.id),
                level: Set(current_level as i32),
                roadmap_id: Set(roadmap.id),
                ..Default::default()
            }.insert(&conn).await
                .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;
            
        }
        current_level += 1;

    }

    roadmap_user::ActiveModel{
        user_id: Set(target.id),
        level: Set(current_level as i32),
        roadmap_id: Set(roadmap.id),
        ..Default::default()
    }.insert(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    Ok(StatusCode::OK)

}

pub async fn roadmap_get(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<user::Model>
)-> Result<Json<RoadmapModel>, APIError>{
    
    let roadmap = roadmap::Entity::find()
    .filter(roadmap::Column::UserId.eq(user.id))
    .one(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(APIError { error_code: None, message: "Resource Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    
    let roadmap_user_models: Vec<LevelModel> = roadmap_user::Entity::find().filter(roadmap_user::Column::RoadmapId.eq(roadmap.id))
    .find_with_related(user::Entity)
    .all(&conn).await.unwrap().into_iter()
    .map(|item| {

        let temp: Vec<UserMicroModel> = item.1
        .into_iter().map(|item2| UserMicroModel::from(item2)).collect();

        LevelModel{
            id: item.0.id,
            level: item.0.level,
            user: temp[0].clone()
        }
    }).collect();
    
    let data = RoadmapModel{
        id: roadmap.id,
        uuid: roadmap.uuid,
        levels: roadmap_user_models,
        target: roadmap.target_id,
        created_at: roadmap.created_at,
        modified_at: roadmap.modified_at,
    };
    Ok(Json(data))
    
}
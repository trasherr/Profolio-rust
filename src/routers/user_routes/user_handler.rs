use axum::{ http::StatusCode, Json, Extension, extract::Path };
use chrono::Utc;
use entity::{user, user_technology, technology};
use sea_orm::{ DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait, LoaderTrait, Condition, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{models::{user_model::{UserModel, UserMicroModel}, tech_model::TechModel}, utils::api_error::APIError};

#[derive(Deserialize)]
pub struct UserSubDetails{
    name: String,
    ctc: Option<i32>,
    phone: String,
    profession: Option<String>,
    experience: i32,
    company: Option<String>,
    others: Option<String>,
    github: Option<String>,
    linkedin: Option<String>,    
}


#[derive(Deserialize,Serialize,Debug)]
pub struct TechnologyResponse{
    uuid: Uuid,
    title: String,
    normalized_title: String,
}


#[derive(Deserialize,Serialize,Debug)]
pub struct Filters{
    lower_ctc: Option<i32>,
    upper_ctc: Option<i32>,
    custom_tech: Option<Vec<Uuid>>,
    company: Option<String>
}


pub async fn update(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>, 
    Json(user_data): Json<UserSubDetails>
) -> Result<(), APIError> {

    
    let mut u: user::ActiveModel = user::Entity::find_by_id(identity.id)
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(APIError { error_code: None, message: "Resource Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?
    .into();

    u.name = Set(user_data.name);
    u.phone = Set(user_data.phone);

    u.company = Set(user_data.company);
    u.ctc = Set(user_data.ctc.unwrap_or(0));
  
    u.profession = Set(user_data.profession);
    u.experience = Set(user_data.experience);
    u.others = Set(user_data.others);
    u.github = Set(user_data.github);
    u.linkedin = Set(user_data.linkedin);

    u.update(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    Ok(()) 

}

pub async fn add_tech(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>, 
    Json(technologies): Json<Vec<Uuid>>
) -> Result<StatusCode,APIError> {

    let techs = technology::Entity::find()
    .filter(technology::Column::Uuid.is_in(technologies))
    .all(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;


    let mut user_techs: Vec<user_technology::ActiveModel> = [].to_vec();
    for (_, item) in techs.into_iter().enumerate() {
        let temp: user_technology::ActiveModel = user_technology::ActiveModel { 
            user_id: Set(identity.id),
            technology_id: Set(item.id),
            score: Set(1.0),
            ..Default::default()
        };
        user_techs.push(temp);
    }

    let _res= user_technology::Entity::insert_many(user_techs).exec(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    Ok(StatusCode::OK)
}


pub async fn user(Extension(identity): Extension<user::Model>) -> Result<Json<UserModel>,APIError>{

    let data = UserModel::from(identity);
    Ok(Json(data))

}

pub async fn user_tech(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>
) -> Result<Json<Vec<TechnologyResponse>>,APIError>{

    let _res = user_technology::Entity::find().filter(user_technology::Column::UserId.eq(identity.id)).all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;

    let res: Vec<TechnologyResponse>= _res.load_one(technology::Entity, &conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?

    .into_iter()
    .map(|item| -> TechnologyResponse { 
        let temp = item.as_ref().unwrap();
        TechnologyResponse { 
            uuid: temp.uuid, 
            title: temp.title.clone(),
            normalized_title: temp.normalized_title.clone()
        } 
    }).collect();
    
    Ok(Json(res))

}

pub async fn get_target_post(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>,
    Json(filters): Json<Filters>

) -> Result<Json<Vec<UserMicroModel>>, APIError>{

    let mut condition = Condition::all().add(user::Column::IsCaption.eq(true));

    if filters.company != None {
        let s = filters.company.unwrap_or("".to_string());
        condition = condition.add(user::Column::Company.like(&format!("%{}%",s)));
    }

    if filters.lower_ctc != None {
        condition = condition.add(user::Column::Ctc.gte(filters.lower_ctc));
    }

    if filters.upper_ctc != None {
        condition = condition.add(user::Column::Ctc.lte(filters.upper_ctc));
    }
    condition = condition.add(user::Column::Ctc.gte(identity.ctc+2));

    if filters.custom_tech != None {
        
        let techs: Vec<i32> = technology::Entity::find()
        .filter(technology::Column::Uuid.is_in(filters.custom_tech))
        .all(&conn)
        .await
        .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::NOT_FOUND})?
        .into_iter().map(|item| item.id)
        .collect();

        let target_ids: Vec<i32> = user_technology::Entity::find()
        .filter(
            Condition::all()
            .add(user_technology::Column::TechnologyId.is_in(techs))
        )
        .all(&conn).await
        .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::NOT_FOUND})?
        .into_iter().map(|item| item.user_id).collect();

        condition = condition.add(user::Column::Id.is_in(target_ids));
    }

    let users = user::Entity::find()
    .filter(condition)
    .order_by_desc(user::Column::Ctc)
    .all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::NOT_FOUND})?
    .into_iter()
    .map(|item|  UserMicroModel::from(item))
    .collect();

    Ok(Json(users))

}



pub async fn get_apply_caption(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(identity): Extension<user::Model>

) -> Result<(), APIError>{
    let mut u: user::ActiveModel = user::Entity::find_by_id(identity.id)
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .unwrap()
    .into();

    u.is_caption_applied = Set(true);

    u.update(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;
    Ok(())
}


pub async fn get_other_user(
    Extension(conn): Extension<DatabaseConnection>, 
    Path(user_uuid): Path<Uuid>,
) -> Result<Json<UserModel>,APIError>{

    let data = user::Entity::find()
    .filter(user::Column::Uuid.eq(user_uuid))
    .one(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .ok_or(APIError { error_code: None, message: "Resource Not Found".to_owned(), status_code: StatusCode::NOT_FOUND})?;

    let techs: Vec<TechModel> = user_technology::Entity::find()
    .filter(user_technology::Column::UserId.eq(data.id))
    .find_with_related(technology::Entity).all(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .into_iter().map(|item| {
        let temp = item.1.first().unwrap();
        TechModel{
            uuid: temp.uuid,
            title: temp.title.to_owned(),
            normalized_title: temp.normalized_title.to_owned()
        }
    })
    .collect();

    let mut user_data = UserModel::from(data);
    user_data.tech = Some(techs);

    Ok(Json(user_data))

}
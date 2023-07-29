use axum::{ http::StatusCode, Json, Extension };
use entity::{user::{self, Model}, user_technology, technology};
use sea_orm::{ DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait, LoaderTrait, Condition, QueryOrder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{models::user_model::{UserModel, UserMicroModel}, utils::api_error::APIError};

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


#[derive(Deserialize,Serialize,Debug)]
pub struct Filters{
    lower_ctc: Option<i32>,
    upper_ctc: Option<i32>,
    custom_tech: Option<Vec<Uuid>>,
    company: Option<String>
}


pub async fn update(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>, 
    Json(user_data): Json<UserSubDetails>
) -> Result<StatusCode, APIError> {

    
    let mut u: user::ActiveModel = user::Entity::find_by_id(user.id)
    .one(&conn)
    .await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?
    .unwrap()
    .into();

    u.company = Set(user_data.company);

    u.ctc = Set(user_data.ctc.unwrap_or(0));
  
    u.profession = Set(user_data.profession);
    u.experience = Set(user_data.experience);

    u.update(&conn).await
    .map_err(|err| APIError { error_code: None, message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR})?;
    // let u: user::Model = u.update(&conn).await.unwrap();
    // (StatusCode::OK, Json(json!({
    //     "email": u.email, 
    //     "name": u.name, 
    //     "uuid": u.uuid,
    //     "ctc": u.ctc,
    //     "profession": u.profession,
    //     "experience": u.experience,
    //     "company": u.company
    // } ))) 
    Ok(StatusCode::OK) 

}

pub async fn add_tech(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>, 
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
            user_id: Set(user.id),
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


pub async fn user(Extension(user): Extension<Model>) -> Result<Json<UserModel>,APIError>{

    let data = UserModel {
        id: user.id,
        name: user.name,
        email: user.email,
        phone: user.phone,
        phone_code: user.phone_code,
        ctc: user.ctc,
        profession: user.profession,
        experience: user.experience,
        company: user.company,

        uuid: user.uuid,
        linkedin: user.linkedin,
        github: user.github,
        others: user.others
    };

    Ok(Json(data))

}

pub async fn user_tech(
    Extension(conn): Extension<DatabaseConnection>, 
    Extension(user): Extension<Model>
) -> Result<Json<Vec<TechnologyResponse>>,APIError>{

    let _res = user_technology::Entity::find().filter(user_technology::Column::UserId.eq(user.id)).all(&conn).await
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
    Extension(identity): Extension<Model>,
    Json(filters): Json<Filters>

) -> Result<Json<Vec<UserMicroModel>>, APIError>{

    let mut condition = Condition::all();

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

        let target_ids: Vec<i32> = user_technology::Entity::find().filter(
            Condition::all()
            .add(user_technology::Column::TechnologyId.is_in(techs))
        ).all(&conn).await
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
    .map(|item|  UserMicroModel { name: item.name, company: item.company, ctc: item.ctc, uuid: item.uuid })
    .collect();

    Ok(Json(users))

}
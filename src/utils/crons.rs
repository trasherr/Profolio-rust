// use chrono::Utc;
// use entity::{user, leagues ,technology};
// use sea_orm::{ Set, ActiveModelTrait, DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait, Condition};

// use uuid::Uuid;

// pub async fn create_leaderboard(conn: &DatabaseConnection) {

   

//     community::Entity::delete_many().exec(conn).await.unwrap();
//     community_user::Entity::delete_many().exec(conn).await.unwrap();

//     for techs in technology::Entity::find().all(conn).await.unwrap(){

//         let leag = leagues::Entity::find().all(conn).await.unwrap();
    
//         let i = 0;    
//         for league_item in leag {
    
//             let mut users = user::Entity::find()
//             .filter(
//                 Condition::all()
//                 .add(user::Column::Ctc.between(league_item.ctc_lower, league_item.ctc_upper))
//                 .add(user::Column::TechId.eq(techs.id))
//             )
//             .order_by_desc(user::Column::Ctc).paginate(conn, league_item.size as u64);
    
//             //+++++++++++++++++++++++
//             let comm = community::ActiveModel {
//                 title: Set(league_item.title + &format!(" {}",i)),
//                 ctc_range: Set(0.0),
//                 tech_id: Set(techs.id),
//                 uuid: Set(Uuid::new_v4()),
//                 ..Default::default()
//             }.insert(conn).await.unwrap();
//             //+++++++++++++++++++++++++++++++


//             let mut range: f64 = 0.0;
//             while let Some(page) = users.fetch_and_next().await.unwrap() {
    
//                 for item in page {
    
//                     community_user::ActiveModel {
//                         user_id: Set(item.id),
//                         community_id: Set(comm.id),
//                         created_at: Set(Utc::now().naive_utc()),
//                         ..Default::default()
//                     }.insert(conn).await.unwrap();
    
//                     range += item.ctc as f64;
//                 }
    
//                 let comm_for_update = community::Entity::find_by_id(comm.id).one(conn).await.unwrap();
    
//                 let mut comm_update: community::ActiveModel = comm_for_update.unwrap().into();
//                 comm_update.ctc_range = Set(range);
//                 comm_update.update(conn).await.unwrap();
                
//             }
//         }

//     }


// }



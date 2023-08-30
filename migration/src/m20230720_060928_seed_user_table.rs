use entity::review_slot;
use entity::technology;
use entity::user_technology;
use rand::Rng;
use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Set;
use sea_orm_migration::sea_orm::entity::*;
use entity::user;
use sha2::Digest;
use sha2::Sha256;
use sha2::digest;
use uuid::Uuid;
use chrono::Utc;
use chrono::Duration;
// fake
use fake::Fake;
#[derive(DeriveMigrationName)]
pub struct Migration;



fn create_hash<D>(msg: &str, mut hasher: D) -> String
where
    D: Digest,
    digest::Output<D>: std::fmt::LowerHex,
{
    hasher.update(msg);
    format!("{:x}", hasher.finalize())
}


#[async_trait]

impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        use fake::faker::name::raw::*;
        use fake::faker::internet::en::{FreeEmail, Password};
        use fake::faker::phone_number::en::CellNumber;
        use fake::faker::company::en::{CompanyName, Profession};
        use fake::locales::*;

        let me = user::ActiveModel { 
            name: Set("Prakhar Singh".to_owned()),
            uuid: Set(Uuid::new_v4()),
            email: Set("prakhar@gmail.com".to_owned()) ,
            password: Set(create_hash("12345678", Sha256::default())),
            phone: Set(CellNumber().fake()),
            phone_code: ActiveValue::set(91),
            experience: Set(rand::thread_rng().gen_range(0..20)),
            ctc: Set(rand::thread_rng().gen_range(0..20)),
            company: Set(CompanyName().fake()),
            profession: Set(Some(Profession().fake())),

            ..Default::default()
        };
        me.insert(db).await.unwrap();

        

        for i in 0..100{
            let user = user::ActiveModel { 
                name: Set(Name(EN).fake()),
                uuid: Set(Uuid::new_v4()),
                email: Set(FreeEmail().fake()) ,
                password: Set(create_hash(&Password(8..12).fake::<String>(), Sha256::default())),
                phone: Set(CellNumber().fake::<String>().to_string().replace("(","").replace(")","").replace("-","")),
                phone_code: ActiveValue::set(91),
                experience: Set(rand::thread_rng().gen_range(0..20)),
                company: Set(Some(CompanyName().fake())),
                ctc: Set(rand::thread_rng().gen_range(3..20)),
                profession: Set(Some(Profession().fake())),
                is_caption: Set(i % 2 == 0),
                is_caption_applied: Set(i % 2 == 0),

                ..Default::default()
            
            };
            if i % 2 == 0 {
                let user_data  = user.insert(db).await.unwrap();
                review_slot::ActiveModel {
                    caption_id: Set(user_data.id),
                    uuid: Set(Uuid::new_v4()),
                    slot_time: Set((Utc::now() + Duration::seconds(48 * 3600)).naive_utc()),
                    ..Default::default()
                }.insert(db).await.unwrap();

                review_slot::ActiveModel {
                    caption_id: Set(user_data.id),
                    uuid: Set(Uuid::new_v4()),
                    slot_time: Set((Utc::now() + Duration::seconds(72 * 3600)).naive_utc()),
                    ..Default::default()
                }.insert(db).await.unwrap();


                let tech_user =user_technology::ActiveModel{ 
                    technology_id: Set(rand::thread_rng().gen_range(1..5)) , 
                    user_id:Set( user_data.id), 
                    score: Set(1.0),
                    ..Default::default()
                };

                tech_user.insert(db).await.unwrap();
            }
           
        }

        for _ in 0..20{
            let user = user::ActiveModel { 
                name: Set(Name(EN).fake()),
                uuid: Set(Uuid::new_v4()),
                email: Set(FreeEmail().fake()) ,
                password: Set(Password(8..12).fake()),
                phone: Set(CellNumber().fake()),
                phone_code: ActiveValue::set(91),
                experience: Set(0),
                ctc: Set(0),

                ..Default::default()
            
            };
            user.insert(db).await.unwrap();
        }

        Ok(())    
    }


}

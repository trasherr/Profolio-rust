use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Set;
use sea_orm_migration::sea_orm::entity::*;
use uuid::Uuid;
use entity::technology;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]

impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        technology::ActiveModel { 
            title: Set("Axum".to_owned()),
            normalized_title: Set("AXUM".to_owned()),
            uuid: Set(Uuid::new_v4()),
            ..Default::default()
        
        }.insert(db).await.unwrap();

        technology::ActiveModel { 
            title: Set("Angular".to_owned()),
            normalized_title: Set("ANGULAR".to_owned()),
            uuid: Set(Uuid::new_v4()),
            ..Default::default()
        
        }.insert(db).await.unwrap();

        technology::ActiveModel { 
            title: Set(".Net".to_owned()),
            normalized_title: Set(".NET".to_owned()),
            uuid: Set(Uuid::new_v4()),
            ..Default::default()
        
        }.insert(db).await.unwrap();

        technology::ActiveModel { 
            title: Set("NodeJs".to_owned()),
            normalized_title: Set("NODEJS".to_owned()),
            uuid: Set(Uuid::new_v4()),
            ..Default::default()
        
        }.insert(db).await.unwrap();

        technology::ActiveModel { 
            title: Set("Rust".to_owned()),
            normalized_title: Set("RUST".to_owned()),
            uuid: Set(Uuid::new_v4()),
            ..Default::default()
        
        }.insert(db).await.unwrap();

        // technology::insert_many(vec![tech0, tech1,tech2,tech3,tech4]).exec(&db).await?;
        // tech0
        Ok(())
    
    }

    // async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    //     // Replace the sample below with your own migration scripts
    //     todo!();

    //     // manager
    //     //     .drop_table(Table::drop().table(Post::Table).to_owned())
    //     //     .await
    // }
}

use sea_orm_migration::{prelude::*, async_trait::async_trait, sea_orm::{Set, ActiveModelTrait}};
use entity::leagues;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        let db = manager.get_connection();

        leagues::ActiveModel {
            title: Set("Earth".to_owned()),
            ctc_lower: Set(0),
            ctc_upper: Set(3),
            size: Set(25),
            ..Default::default()

        }.insert(db).await.unwrap();

        leagues::ActiveModel {
            title: Set("Solar".to_owned()),
            ctc_lower: Set(3),
            ctc_upper: Set(6),
            size: Set(25),
            ..Default::default()

        }.insert(db).await.unwrap();

        leagues::ActiveModel {
            title: Set("Triangulum".to_owned()),
            ctc_lower: Set(6),
            ctc_upper: Set(12),
            size: Set(20),
            ..Default::default()

        }.insert(db).await.unwrap();


        leagues::ActiveModel {
            title: Set("Milky Way".to_owned()),
            ctc_lower: Set(6),
            ctc_upper: Set(12),
            size: Set(25),
            ..Default::default()

        }.insert(db).await.unwrap();

        leagues::ActiveModel {
            title: Set("Andromeda".to_owned()),
            ctc_lower: Set(12),
            ctc_upper: Set(18),
            size: Set(15),
            ..Default::default()

        }.insert(db).await.unwrap();


        leagues::ActiveModel {
            title: Set("Laniakea".to_owned()),
            ctc_lower: Set(18),
            ctc_upper: Set(26),
            size: Set(15),
            ..Default::default()

        }.insert(db).await.unwrap();


        leagues::ActiveModel {
            title: Set("Pisces–Cetus".to_owned()),
            ctc_lower: Set(26),
            ctc_upper: Set(1000),
            size: Set(10),
            ..Default::default()

        }.insert(db).await.unwrap();

        Ok(())
    }
}

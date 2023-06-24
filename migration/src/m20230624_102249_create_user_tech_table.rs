use sea_orm_migration::prelude::*;

use crate::m20230623_113733_create_tech_table::Technology;
use crate::m20220101_000001_create_table::User;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(UserTechnology::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTechnology::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTechnology::TechnologyId).integer().not_null())
                    .col(ColumnDef::new(UserTechnology::UserId).integer().not_null())
                    .col(ColumnDef::new(UserTechnology::Score).double().not_null())
                    .foreign_key(ForeignKey::create().name("fk-user_tech-tech-id").from(UserTechnology::Table,UserTechnology::TechnologyId).to(Technology::Table, Technology::Id))
                    .foreign_key(ForeignKey::create().name("fk-user_tech-user-id").from(UserTechnology::Table,UserTechnology::UserId).to(User::Table, User::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(UserTechnology::Table).to_owned())
            .await
    }
}



/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserTechnology {
    Table,
    Id,
    TechnologyId,
    UserId,
    Score
}

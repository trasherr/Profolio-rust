use sea_orm_migration::prelude::*;
use crate::m20220101_000001_create_table::User;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Community::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Community::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Community::Uuid).uuid().not_null())
                    .col(ColumnDef::new(Community::UserId).integer().not_null())
                    .col(ColumnDef::new(Community::Title).string().not_null())
                    .col(ColumnDef::new(Community::Description).string())
                    .foreign_key(ForeignKey::create().name("fk-community-user-id").from(Community::Table,Community::UserId).to(User::Table, User::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
    

        manager
            .drop_table(Table::drop().table(Community::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Community {
    Table,
    Id,
    Uuid,
    UserId,
    Title,
    Description
}

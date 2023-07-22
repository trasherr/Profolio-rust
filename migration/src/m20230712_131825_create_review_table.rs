use sea_orm_migration::prelude::*;

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
                    .table(Review::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Review::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Review::UserId).integer().not_null())
                    .col(ColumnDef::new(Review::CaptionId).integer().not_null())
                    .col(ColumnDef::new(Review::Rating).integer().not_null())
                    .col(ColumnDef::new(Review::Text).string())
                    // .foreign_key(ForeignKey::create().name("fk-review-user-id").from(Review::Table,Review::UserId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().name("fk-review_caption-target-id").from(Review::Table,Review::CaptionId).to(User::Table, User::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Review {
    Table,
    Id,
    UserId,
    CaptionId,
    Rating,
    Text
}

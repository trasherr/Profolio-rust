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
                    .table(ReviewSlot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReviewSlot::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReviewSlot::CaptionId).integer().not_null())
                    .col(ColumnDef::new(ReviewSlot::UserId).integer())
                    .col(ColumnDef::new(ReviewSlot::Uuid).uuid().not_null().unique_key())
                    .col(ColumnDef::new(ReviewSlot::SlotTime).date_time().not_null())
                    
                    // .foreign_key(ForeignKey::create().name("fk-review_slot-user-id").from(ReviewSlot::Table,ReviewSlot::UserId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().name("fk-review_slot_caption-target-id").from(ReviewSlot::Table,ReviewSlot::CaptionId).to(User::Table, User::Id))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(ReviewSlot::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ReviewSlot {
    Table,
    Id,
    Uuid,
    CaptionId,
    SlotTime,
    UserId
}

use sea_orm_migration::prelude::*;

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

use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::User;
use crate::m20230713_162119_create_slots::ReviewSlot;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Order::UserId).integer().not_null())
                    .col(ColumnDef::new(Order::SlotId).integer().not_null())
                    .col(ColumnDef::new(Order::OrderId).string().not_null())
                    .col(ColumnDef::new(Order::Currency).string().not_null())
                    .col(ColumnDef::new(Order::Amount).integer().not_null())
                    .col(ColumnDef::new(Order::AmountPaid).integer().not_null())
                    .col(ColumnDef::new(Order::AmountDue).integer().not_null())
                    .col(ColumnDef::new(Order::Receipt).string().not_null())
                    .col(ColumnDef::new(Order::OfferId).string())
                    .col(ColumnDef::new(Order::Status).string().not_null())
                    .col(ColumnDef::new(Order::Attempts).small_unsigned().not_null())
                    .col(ColumnDef::new(Order::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Order::CreatedAt).date_time().not_null())

                    .foreign_key(ForeignKey::create().name("fk-order-user-id").from(Order::Table,Order::UserId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().name("fk-order-slot-id").from(Order::Table,Order::SlotId).to(ReviewSlot::Table, ReviewSlot::Id))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scrip
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Order {
    Table,
    Id,
    UserId,
    SlotId,
    OrderId,
    Currency,
    Amount,
    AmountPaid,
    AmountDue,
    Receipt,
    OfferId,
    Status,
    Attempts,
    UpdatedAt,
    CreatedAt,
}

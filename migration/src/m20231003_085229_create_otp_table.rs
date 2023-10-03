use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Otp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Otp::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Otp::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Otp::UserId).integer().not_null().unique_key())
                    .col(ColumnDef::new(Otp::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Otp::Expire).string().date_time().not_null())
                    .foreign_key(ForeignKey::create().name("fk-otp-user-id").from(Otp::Table,Otp::UserId).to(User::Table, User::Id))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Otp::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Otp {
    Table,
    Id,
    UserId,
    Code,
    CreatedAt,
    Expire
}

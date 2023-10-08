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
                    .table(RemoveUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RemoveUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RemoveUser::UserId).integer().not_null().unique_key())
                    .col(ColumnDef::new(RemoveUser::CreatedAt).date_time().not_null())
                    .foreign_key(ForeignKey::create().name("fk-remove-user-user-id").from(RemoveUser::Table,RemoveUser::UserId).to(User::Table, User::Id))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(RemoveUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum RemoveUser {
    Table,
    Id,
    UserId,
    CreatedAt,
}

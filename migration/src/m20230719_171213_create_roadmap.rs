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
                    .table(Roadmap::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roadmap::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Roadmap::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key()
                    )
                    .col(ColumnDef::new(Roadmap::UserId).integer().not_null())
                    .col(ColumnDef::new(Roadmap::TargetId).integer().not_null())
                    .col(ColumnDef::new(Roadmap::ModifiedAt).date_time().not_null())
                    .col(ColumnDef::new(Roadmap::CreatedAt).date_time().not_null())
                    .foreign_key(ForeignKey::create().name("fk-roadmap-user-id").from(Roadmap::Table,Roadmap::UserId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().name("fk-roadmap-target-id").from(Roadmap::Table,Roadmap::TargetId).to(User::Table, User::Id))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Roadmap::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Roadmap {
    Table,
    Id,
    Uuid,
    UserId,
    TargetId,
    ModifiedAt,
    CreatedAt
}

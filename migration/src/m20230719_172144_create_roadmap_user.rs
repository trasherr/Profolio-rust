use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_table::User, m20230719_171213_create_roadmap::Roadmap};

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
                    .table(RoadmapUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoadmapUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RoadmapUser::Level).integer().not_null())
                    .col(ColumnDef::new(RoadmapUser::UserId).integer().not_null())
                    .col(ColumnDef::new(RoadmapUser::RoadmapId).integer().not_null())
                    .foreign_key(ForeignKey::create().name("fk-RoadmapUser-user-id").from(RoadmapUser::Table,RoadmapUser::UserId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().name("fk-Roadmap-roadmapuser-user-id").from(RoadmapUser::Table,RoadmapUser::RoadmapId).to(Roadmap::Table, Roadmap::Id))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(RoadmapUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum RoadmapUser {
    Table,
    Id,
    Level,
    RoadmapId,
    UserId,
}

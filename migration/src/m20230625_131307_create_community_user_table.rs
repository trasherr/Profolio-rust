use sea_orm_migration::{prelude::*, sea_orm::EnumIter};

use crate::{m20220101_000001_create_table::User, m20230625_113658_create_community_table::Community};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts


        manager
            .create_table(
                Table::create()
                    .table(CommunityUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommunityUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CommunityUser::UserId).integer().not_null())
                    .col(ColumnDef::new(CommunityUser::Caption1).integer())
                    .col(ColumnDef::new(CommunityUser::Caption2).integer())
                    .col(ColumnDef::new(CommunityUser::Caption3).integer())
                    .col(ColumnDef::new(CommunityUser::Caption4).integer())
                    .col(ColumnDef::new(CommunityUser::Caption5).integer())
                    .col(ColumnDef::new(CommunityUser::CommunityId).integer().not_null())
                    .col(ColumnDef::new(CommunityUser::Type).string())
                    .col(ColumnDef::new(CommunityUser::CreatedAt).date_time().not_null())
                    .foreign_key(ForeignKey::create().name("fk-user_community_user-user-id").from(CommunityUser::Table,CommunityUser::UserId).to(User::Table, User::Id))
                    .foreign_key(ForeignKey::create().name("fk-user_community-community-id").from(CommunityUser::Table,CommunityUser::CommunityId).to(Community::Table, Community::Id))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(CommunityUser::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CommunityUser {
    Table,
    Id,
    UserId,
    CommunityId,
    Type,
    Caption1,
    Caption2,
    Caption3,
    Caption4,
    Caption5,
    CreatedAt
}

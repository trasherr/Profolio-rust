use sea_orm_migration::prelude::*;
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
                    .col(ColumnDef::new(Community::TechId).integer().not_null())
                    .col(ColumnDef::new(Community::CtcRange).double().not_null())
                    .col(ColumnDef::new(Community::Title).string().not_null())
                    .col(ColumnDef::new(Community::Description).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Community::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Community {
    Table,
    Id,
    Uuid,
    TechId,
    CtcRange,
    Title,
    Description
}

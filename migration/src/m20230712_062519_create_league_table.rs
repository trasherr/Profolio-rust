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
                    .table(Leagues::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Leagues::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Leagues::Title).string().not_null())
                    .col(ColumnDef::new(Leagues::CtcLower).integer().not_null())
                    .col(ColumnDef::new(Leagues::CtcUpper).integer().not_null())
                    .col(ColumnDef::new(Leagues::Size).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
  

        manager
            .drop_table(Table::drop().table(Leagues::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Leagues {
    Table,
    Id,
    Title,
    CtcLower,
    CtcUpper,
    Size,
}

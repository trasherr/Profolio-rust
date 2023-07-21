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
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::Uuid)
                            .uuid()
                            .not_null()
                            .unique_key()
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Phone).string().not_null())
                    .col(ColumnDef::new(User::PhoneCode).integer().not_null())
                    .col(ColumnDef::new(User::Profession).string())
                    .col(ColumnDef::new(User::Ctc).integer().default(0).not_null())
                    .col(ColumnDef::new(User::Experience).integer().default(0).not_null())
                    .col(ColumnDef::new(User::TotalRating).integer().default(0).not_null())
                    .col(ColumnDef::new(User::TotalReviews).integer().default(0).not_null())
                    .col(ColumnDef::new(User::Company).string())
                    .col(ColumnDef::new(User::Linkedin).string())
                    .col(ColumnDef::new(User::Github).string())
                    .col(ColumnDef::new(User::Others).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Uuid,
    Name,
    Email,
    Password,
    Phone,
    PhoneCode,
    Ctc,
    Profession,
    Experience,
    TotalRating,
    TotalReviews,
    Company,
    Linkedin,
    Github,
    Others
}

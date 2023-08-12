use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(OrderSignature::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderSignature::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderSignature::RazorpayPaymentId).string().not_null())
                    .col(ColumnDef::new(OrderSignature::RazorpayOrderId).string().not_null())
                    .col(ColumnDef::new(OrderSignature::RazorpaySignature).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .drop_table(Table::drop().table(OrderSignature::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum OrderSignature {
    Table,
    Id,
    RazorpayPaymentId,
    RazorpayOrderId,
    RazorpaySignature
}

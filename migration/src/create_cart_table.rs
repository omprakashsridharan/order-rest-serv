use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cart::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cart::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cart::UserId).integer().not_null())
                    .col(ColumnDef::new(Cart::ProductId).integer().not_null())
                    .col(ColumnDef::new(Cart::OrderRequestId).uuid().null())
                    .col(
                        ColumnDef::new(Cart::CreatedAt)
                            .not_null()
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_string()),
                    )
                    .col(
                        ColumnDef::new(Cart::UpdatedAt)
                            .not_null()
                            .date_time()
                            .extra(
                                "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_string(),
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cart::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Cart {
    Table,
    Id,
    UserId,
    ProductId,
    OrderRequestId,
    CreatedAt,
    UpdatedAt,
}

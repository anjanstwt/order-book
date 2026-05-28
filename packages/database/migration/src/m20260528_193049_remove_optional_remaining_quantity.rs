use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Order::Table)
                    .modify_column(
                        ColumnDef::new(Order::RemainingQuantity)
                            .integer()
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Order::Table)
                    .modify_column(ColumnDef::new(Order::RemainingQuantity).integer().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Order {
    Table,
    RemainingQuantity,
}

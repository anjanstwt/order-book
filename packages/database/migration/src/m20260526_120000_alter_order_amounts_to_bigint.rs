use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Order::Table)
                    .modify_column(ColumnDef::new(Order::Quantity).big_integer().not_null())
                    .modify_column(ColumnDef::new(Order::Tick).big_integer().not_null())
                    .modify_column(ColumnDef::new(Order::Idx).big_integer())
                    .modify_column(ColumnDef::new(Order::RemainingQuantity).big_integer())
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
                    .modify_column(ColumnDef::new(Order::Quantity).integer().not_null())
                    .modify_column(ColumnDef::new(Order::Tick).integer().not_null())
                    .modify_column(ColumnDef::new(Order::Idx).integer())
                    .modify_column(ColumnDef::new(Order::RemainingQuantity).integer())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Quantity,
    Tick,
    Idx,
    RemainingQuantity,
}

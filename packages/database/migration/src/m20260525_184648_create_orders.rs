use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Market::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Market::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Market::CurrencyA).string().not_null())
                    .col(ColumnDef::new(Market::CurrencyB).string().not_null())
                    .col(
                        ColumnDef::new(Market::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(OrderStatus::Table)
                    .values([
                        OrderStatus::New,
                        OrderStatus::Resting,
                        OrderStatus::Filled,
                        OrderStatus::PartiallyFilled,
                        OrderStatus::Canceled,
                        OrderStatus::Rejected,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Order::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Order::MarketId).uuid().not_null())
                    .col(ColumnDef::new(Order::Quantity).integer().not_null())
                    .col(ColumnDef::new(Order::Tick).integer().not_null())
                    .col(ColumnDef::new(Order::Idx).integer())
                    .col(
                        ColumnDef::new(Order::Status)
                            .enumeration(
                                OrderStatus::Table,
                                [
                                    OrderStatus::New,
                                    OrderStatus::Resting,
                                    OrderStatus::Filled,
                                    OrderStatus::PartiallyFilled,
                                    OrderStatus::Canceled,
                                    OrderStatus::Rejected,
                                ],
                            )
                            .not_null()
                            .default("new"),
                    )
                    .col(ColumnDef::new(Order::RemainingQuantity).integer())
                    .col(ColumnDef::new(Order::CreatedAt).date_time())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-order-market")
                            .from(Order::Table, Order::MarketId)
                            .to(Market::Table, Market::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(OrderStatus::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Market::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Market {
    Table,
    Id,
    CurrencyA,
    CurrencyB,
    CreatedAt,
}

#[derive(DeriveIden)]
enum OrderStatus {
    Table,
    New,
    Resting,
    Filled,
    PartiallyFilled,
    Rejected,
    Canceled,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    MarketId,
    Quantity,
    Tick,
    Status,
    Idx,
    RemainingQuantity,
    CreatedAt,
}

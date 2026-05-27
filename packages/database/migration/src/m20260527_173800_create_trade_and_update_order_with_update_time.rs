use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Side::Table)
                    .values([Side::Bid, Side::Ask])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Trade::Table)
                    .col(pk_uuid(Trade::Id).default(Expr::cust("gen_random_uuid()")))
                    .col(uuid(Trade::MarketId).not_null())
                    .col(big_integer(Trade::Sequence).not_null())
                    .col(uuid(Trade::MakerOrderId).not_null())
                    .col(uuid(Trade::TakerOrderId).not_null())
                    .col(
                        ColumnDef::new(Trade::MakerSide)
                            .enumeration(Side::Table, [Side::Bid, Side::Ask]),
                    )
                    .col(big_integer(Trade::Tick).not_null())
                    .col(big_integer(Trade::Quantity).not_null())
                    .col(date_time(Trade::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-trade-market")
                            .from(Trade::Table, Trade::MarketId)
                            .to(Market::Table, Market::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-trade-maker-order")
                            .from(Trade::Table, Trade::MakerOrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-trade-taker-order")
                            .from(Trade::Table, Trade::TakerOrderId)
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Order::Table)
                    .add_column(date_time(Order::UpdatedAt).not_null())
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
                    .drop_column(Order::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Trade::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Side::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Trade {
    Table,
    Id,
    MarketId,
    Sequence,
    MakerOrderId,
    TakerOrderId,
    MakerSide,
    Tick,
    Quantity,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Side {
    Table,
    Bid,
    Ask,
}

#[derive(DeriveIden)]
enum Market {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    UpdatedAt,
}

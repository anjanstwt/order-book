use std::sync::Arc;

use chrono::NaiveDateTime;
use database::{
    order,
    sea_orm_active_enums::{OrderStatus, OrderStatus as DbStatus, Side as DbSide},
    trade,
};
use engine::{Side, Status, Trade};
use events::OrderEvent;
use futures::{StreamExt, stream};
use sea_orm::{
    ActiveValue::{Set, Unchanged},
    ColumnTrait, DbErr, EntityTrait, QueryFilter, TransactionTrait,
    prelude::Expr,
    sea_query::OnConflict,
};
use uuid::Uuid;

use crate::Services;

pub struct DbWriter;

impl DbWriter {
    pub async fn add_order(event: OrderEvent, service: Arc<Services>) -> bool {
        match event.status {
            Status::New => false,
            Status::Canceled => Self::cancel_order(&event, &service).await,
            _ => Self::write_order(&event, &service).await,
        }
    }

    async fn write_order(event: &OrderEvent, service: &Arc<Services>) -> bool {
        let now = event.metadata.timestamp.naive_utc();
        let remaining = event.quantity - event.filled_quantity;

        let model = order::ActiveModel {
            id: Set(event.order_id),
            market_id: Set(event.metadata.market_id),
            quantity: Set(event.quantity as i64),
            tick: Set(event.tick.map(|t| t as i32)),
            idx: Set(event.order_idx.map(|i| i as i64)),
            status: Set(Self::map_status(&event.status)),
            remaining_quantity: Set(remaining as i64),
            created_at: Set(Some(now)),
            updated_at: Set(now),
        };

        // idempotent: replays of the same event hit the primary key and are skipped
        let future_order = order::Entity::insert(model)
            .on_conflict(
                OnConflict::column(order::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&service.db);

        if let Some(trades) = &event.trades {
            let (trades_result, order_result) = tokio::join!(
                Self::write_trades(
                    event.metadata.market_id,
                    event.metadata.timestamp.naive_utc(),
                    trades,
                    service
                ),
                future_order,
            );

            match order_result {
                Ok(_) => {}
                // do_nothing returns this when the row already existed, a benign replay
                Err(DbErr::RecordNotInserted) => {}
                Err(e) => {
                    eprintln!("failed to write order {}: {e}", event.order_id);
                    return false;
                }
            }

            return trades_result;
        }

        match future_order.await {
            Ok(_) => {}
            // do_nothing returns this when the row already existed, a benign replay
            Err(DbErr::RecordNotInserted) => {}
            Err(e) => {
                eprintln!("failed to write order {}: {e}", event.order_id);
                return false;
            }
        }

        true
    }

    async fn write_trades(
        market_id: Uuid,
        now: NaiveDateTime,
        trades: &Vec<Trade>,
        services: &Arc<Services>,
    ) -> bool {
        let results = stream::iter(trades.iter().copied())
            .map(|trade| Self::process_trade(market_id, now, trade, Arc::clone(services)))
            .buffer_unordered(200)
            .collect::<Vec<_>>()
            .await;

        for result in results {
            match result {
                Ok(_) | Err(DbErr::RecordNotInserted) => {}
                Err(e) => {
                    eprintln!("failed to write trade: {e}");
                    return false;
                }
            }
        }

        true
    }

    async fn process_trade(
        market_id: Uuid,
        now: NaiveDateTime,
        trade: Trade,
        services: Arc<Services>,
    ) -> Result<(), DbErr> {
        let txn = services.db.begin().await?;

        let trade_id = Uuid::new_v5(&market_id, &trade.sequence.to_le_bytes());

        let trade_model = trade::ActiveModel {
            id: Set(trade_id),
            market_id: Set(market_id),
            sequence: Set(trade.sequence as i64),
            maker_order_id: Set(trade.maker_order_id),
            taker_order_id: Set(trade.taker_order_id),
            maker_side: Set(Some(Self::map_side(&trade.maker_side))),
            tick: Set(trade.tick as i64),
            quantity: Set(trade.quantity as i64),
            created_at: Set(now),
        };

        trade::Entity::insert(trade_model)
            .on_conflict(
                OnConflict::column(trade::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&txn)
            .await?;

        order::Entity::update_many()
            .col_expr(
                order::Column::RemainingQuantity,
                Expr::col(order::Column::RemainingQuantity).sub(trade.quantity as i64),
            )
            .col_expr(
                order::Column::Status,
                Expr::case(
                    order::Column::RemainingQuantity.eq(trade.quantity as i64),
                    Expr::value(DbStatus::Filled),
                )
                .finally(Expr::value(DbStatus::PartiallyFilled))
                .into(),
            )
            .filter(order::Column::Id.eq(trade.maker_order_id))
            .exec(&txn)
            .await?;

        txn.commit().await?;
        Ok(())
    }

    async fn cancel_order(event: &OrderEvent, service: &Arc<Services>) -> bool {
        let now = event.metadata.timestamp.naive_utc();

        // partial update: only status + updated_at change, matched by primary key
        let model = order::ActiveModel {
            id: Unchanged(event.order_id),
            status: Set(OrderStatus::Canceled),
            updated_at: Set(now),
            ..Default::default()
        };

        match order::Entity::update(model).exec(&service.db).await {
            Ok(_) => true,
            Err(DbErr::RecordNotUpdated) => {
                eprintln!("cancel: no order {} found to cancel", event.order_id);
                false
            }
            Err(e) => {
                eprintln!("cancel: failed to update order {}: {e}", event.order_id);
                false
            }
        }
    }

    fn map_status(status: &Status) -> OrderStatus {
        match status {
            Status::New => OrderStatus::New,
            Status::Resting => OrderStatus::Resting,
            Status::PartiallyFilled => OrderStatus::PartiallyFilled,
            Status::Filled => OrderStatus::Filled,
            Status::Canceled => OrderStatus::Canceled,
            Status::Rejected => OrderStatus::Rejected,
        }
    }

    fn map_side(side: &Side) -> DbSide {
        match side {
            Side::Bid => DbSide::Bid,
            Side::Ask => DbSide::Ask,
        }
    }
}

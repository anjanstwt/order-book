use std::sync::Arc;

use database::{
    order,
    sea_orm_active_enums::{OrderStatus, Side as DbSide},
    trade,
};
use engine::{Side, Status};
use events::OrderEvent;
use sea_orm::{
    ActiveValue::{Set, Unchanged},
    DbErr, EntityTrait,
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
            remaining_quantity: Set(Some(remaining as i64)),
            created_at: Set(Some(now)),
            updated_at: Set(now),
        };

        // idempotent: replays of the same event hit the primary key and are skipped
        let res = order::Entity::insert(model)
            .on_conflict(
                OnConflict::column(order::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&service.db)
            .await;

        match res {
            Ok(_) => {}
            // do_nothing returns this when the row already existed, a benign replay
            Err(DbErr::RecordNotInserted) => {}
            Err(e) => {
                eprintln!("failed to write order {}: {e}", event.order_id);
                return false;
            }
        }

        if let Some(trades) = &event.trades {
            return Self::write_trades(event.metadata.market_id, now, trades, service).await;
        }

        true
    }

    async fn write_trades(
        market_id: Uuid,
        now: chrono::NaiveDateTime,
        trades: &[engine::Trade],
        service: &Arc<Services>,
    ) -> bool {
        for t in trades {
            // deterministic id from (market, sequence) so replays don't duplicate trades
            let trade_id = Uuid::new_v5(&market_id, &t.sequence.to_be_bytes());

            let model = trade::ActiveModel {
                id: Set(trade_id),
                market_id: Set(market_id),
                sequence: Set(t.sequence as i64),
                maker_order_id: Set(t.maker_order_id),
                taker_order_id: Set(t.taker_order_id),
                maker_side: Set(Some(Self::map_side(&t.maker_side))),
                tick: Set(t.tick as i64),
                quantity: Set(t.quantity as i64),
                created_at: Set(now),
            };

            let res = trade::Entity::insert(model)
                .on_conflict(
                    OnConflict::column(trade::Column::Id)
                        .do_nothing()
                        .to_owned(),
                )
                .exec(&service.db)
                .await;

            match res {
                Ok(_) | Err(DbErr::RecordNotInserted) => {}
                Err(e) => {
                    eprintln!("failed to write trade seq {}: {e}", t.sequence);
                    return false;
                }
            }
        }
        true
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

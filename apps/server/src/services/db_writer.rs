use std::sync::Arc;

use database::{entities::order, sea_orm_active_enums::OrderStatus};
use events::{Metadata, OrderEvent};
use sea_orm::{ActiveValue::Set, EntityTrait};

use crate::{Services, services};

pub struct DbWriter;

impl DbWriter {
    pub async fn add_order(event: OrderEvent, service: Arc<Services>) -> bool {
        match event {
            OrderEvent::Resting {
                metadata,
                order_id,
                order_idx,
                quantity,
                side,
                tick,
            } => database::order::Entity::insert(order::ActiveModel {
                id: Set(order_id),
                market_id: Set(metadata.market_id),
                quantity: Set(quantity as i64),
                tick: Set(Some(tick as i64)),
                status: Set(OrderStatus::Resting),
                idx: Set(Some(order_idx as i64)),
                remaining_quantity: Set(Some(quantity as i64)),
                created_at: Set(Some(metadata.timestamp.naive_utc())),
            })
            .exec(&service.db)
            .await
            .is_ok(),
            OrderEvent::Filled {
                metadata,
                order_id,
                quantity,
                trades,
                side,
            } => database::order::Entity::insert(order::ActiveModel {
                id: Set(order_id),
                market_id: Set(metadata.market_id),
                quantity: Set(quantity as i64),
                tick: Set(None),
                status: Set(OrderStatus::Filled),
                idx: Set(None),
                remaining_quantity: Set(None),
                created_at: Set(Some(metadata.timestamp.naive_utc())),
            })
            .exec(&service.db)
            .await
            .is_ok(),
            _ => false,
        }
    }
}

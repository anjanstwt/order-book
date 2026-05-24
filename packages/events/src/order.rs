use chrono::{DateTime, Utc};
use engine::{OrderId, Quantity, Trade};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OrderEvent {
    Placed {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        order_idx: usize,
        trades: Vec<Trade>,
    },
    Filled {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        order_idx: usize,
        trades: Vec<Trade>,
    },
    Cancelled {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        order_idx: usize,
    },
    PartiallyFilled {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        filled_quantity: Quantity,
        remaining_quantity: Quantity,
        resting_order_idx: usize,
        trades: Vec<Trade>,
    },
}

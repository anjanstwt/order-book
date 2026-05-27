use chrono::{DateTime, Utc};
use engine::{MatchReport, OrderId, Quantity, Side, Status, Tick, Trade};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub user_id: Uuid,
    pub market_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OrderEvent {
    Resting {
        metadata: Metadata,
        order_id: OrderId,
        order_idx: usize,
        quantity: Quantity,
        tick: Tick,
        side: Side,
    },
    Filled {
        metadata: Metadata,
        order_id: OrderId,
        quantity: Quantity,
        trades: Vec<Trade>,
        side: Side,
    },
    // this just requires the order id rest will be marked as cancelled in db
    Cancelled {
        metadata: Metadata,
        order_id: OrderId,
        order_idx: usize,
        tick: Tick,
        side: Side,
    },
    PartiallyFilled {
        metadata: Metadata,
        order_id: OrderId,
        filled_quantity: Quantity,
        remaining_quantity: Quantity,
        order_idx: usize,
        trades: Vec<Trade>,
        side: Side,
    },
    Rejected {
        metadata: Metadata,
        order_id: OrderId,
        quantity: Quantity,
        side: Side,
    },
}

impl OrderEvent {
    pub fn convert(
        user_id: Uuid,
        market_id: Uuid,
        report: &MatchReport,
        timestamp: DateTime<Utc>,
        order_idx: Option<usize>,
    ) -> Result<Self, String> {
        match report.taker_status {
            Status::Filled => Ok(OrderEvent::Filled {
                metadata: Metadata {
                    user_id,
                    market_id,
                    timestamp,
                },
                order_id: report.taker_order_id,
                trades: report.trades.clone(),
                side: report.taker_side,
            }),
            Status::Resting => Ok(OrderEvent::Resting {
                metadata: Metadata {
                    user_id,
                    market_id,
                    timestamp,
                },
                order_id: report.taker_order_id,
                order_idx: report.resting_order_idx,
                side: report.taker_side,
            }),
            Status::PartiallyFilled => Ok(OrderEvent::PartiallyFilled {
                metadata: Metadata {
                    user_id,
                    market_id,
                    timestamp,
                },
                order_id: report.taker_order_id,
                filled_quantity: report.filled_quantity,
                remaining_quantity: report.remaining_quantity,
                order_idx: report.resting_order_idx,
                trades: report.trades.clone(),
                side: report.taker_side,
            }),
            Status::Canceled => Ok(OrderEvent::Cancelled {
                metadata: Metadata {
                    user_id,
                    market_id,
                    timestamp,
                },
                order_id: report.taker_order_id,
                order_idx,
                side: report.taker_side,
            }),
            Status::Rejected => Ok(OrderEvent::Rejected {
                metadata: Metadata {
                    user_id,
                    market_id,
                    timestamp,
                },
                order_id: report.taker_order_id,
                quantity: report.remaining_quantity,
                side: report.taker_side,
            }),
            Status::New => Err(format!(
                "order {} with New status means it failed to register in engine",
                report.taker_order_id
            )),
        }
    }
}

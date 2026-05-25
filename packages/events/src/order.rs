use chrono::{DateTime, Utc};
use engine::{MatchReport, OrderId, Quantity, Status, Trade};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum OrderEvent {
    Placed {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        order_idx: Option<usize>,
    },
    Filled {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        trades: Vec<Trade>,
    },
    Cancelled {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        order_idx: Option<usize>,
    },
    PartiallyFilled {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        filled_quantity: Quantity,
        remaining_quantity: Quantity,
        resting_order_idx: Option<usize>,
        trades: Vec<Trade>,
    },
    Rejected {
        market_id: Uuid,
        timestamp: DateTime<Utc>,
        order_id: OrderId,
        quantity: Quantity,
    },
}

impl OrderEvent {
    pub fn convert(
        market_id: Uuid,
        report: &MatchReport,
        timestamp: DateTime<Utc>,
        order_idx: Option<usize>,
    ) -> Result<Self, String> {
        match report.taker_status {
            Status::Filled => Ok(OrderEvent::Filled {
                market_id,
                timestamp,
                order_id: report.taker_order_id,
                trades: report.trades.clone(),
            }),
            Status::Resting => Ok(OrderEvent::Placed {
                market_id,
                timestamp,
                order_id: report.taker_order_id,
                order_idx: report.resting_order_idx,
            }),
            Status::PartiallyFilled => Ok(OrderEvent::PartiallyFilled {
                market_id,
                timestamp,
                order_id: report.taker_order_id,
                filled_quantity: report.filled_quantity,
                remaining_quantity: report.remaining_quantity,
                resting_order_idx: report.resting_order_idx,
                trades: report.trades.clone(),
            }),
            Status::Canceled => Ok(OrderEvent::Cancelled {
                market_id,
                timestamp,
                order_id: report.taker_order_id,
                order_idx,
            }),
            Status::Rejected => Ok(OrderEvent::Rejected {
                market_id,
                timestamp,
                order_id: report.taker_order_id,
                quantity: report.remaining_quantity,
            }),
            Status::New => Err(format!(
                "order {} has status New, which has no corresponding OrderEvent",
                report.taker_order_id
            )),
        }
    }
}

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
pub struct OrderEvent {
    pub metadata: Metadata,
    pub order_id: OrderId,
    pub order_idx: Option<usize>,
    pub quantity: Quantity,
    pub side: Side,
    pub tick: Option<Tick>,
    pub status: Status,
    pub filled_quantity: Quantity,
    pub trades: Option<Vec<Trade>>,
}

impl OrderEvent {
    pub fn convert(
        user_id: Uuid,
        market_id: Uuid,
        report: &MatchReport,
        timestamp: DateTime<Utc>,
        tick: Option<Tick>,
    ) -> Result<Self, String> {
        if let Status::New = report.taker_status {
            return Err(format!(
                "order {} with New status means it failed to register in engine",
                report.taker_order_id
            ));
        }

        let trades = if report.trades.is_empty() {
            None
        } else {
            Some(report.trades.clone())
        };

        Ok(OrderEvent {
            metadata: Metadata {
                user_id,
                market_id,
                timestamp,
            },
            order_id: report.taker_order_id,
            order_idx: report.resting_order_idx,
            quantity: report.filled_quantity + report.remaining_quantity,
            side: report.taker_side,
            tick,
            status: report.taker_status,
            filled_quantity: report.filled_quantity,
            trades,
        })
    }
}

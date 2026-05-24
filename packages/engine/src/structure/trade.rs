use serde::{Deserialize, Serialize};

use crate::types::{OrderId, Quantity, Sequence, Side, Tick};

// this will contain the actual trade
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Trade {
    pub sequence: Sequence,
    pub maker_order_id: OrderId,
    pub taker_order_id: OrderId,
    pub maker_side: Side,
    pub tick: Tick,
    pub quantity: Quantity,
}

impl Trade {
    pub fn new(
        sequence: Sequence,
        maker_order_id: OrderId,
        taker_order_id: OrderId,
        maker_side: Side,
        tick: Tick,
        quantity: Quantity,
    ) -> Self {
        Self {
            sequence,
            maker_order_id,
            taker_order_id,
            maker_side,
            tick,
            quantity,
        }
    }
}

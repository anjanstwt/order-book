use crate::{
    Side,
    structure::Trade,
    types::{OrderId, Quantity, Status},
};

// this is the report that user will see about their order
pub struct MatchReport {
    pub taker_order_id: OrderId,
    pub taker_status: Status,
    pub taker_side: Side,
    pub filled_quantity: Quantity,
    pub remaining_quantity: Quantity,
    pub trades: Vec<Trade>,
    pub resting_order_idx: Option<usize>,
}

impl MatchReport {
    pub fn new(
        taker_order_id: OrderId,
        taker_status: Status,
        taker_side: Side,
        filled_quantity: Quantity,
        remaining_quantity: Quantity,
        trades: Vec<Trade>,
        resting_order_idx: Option<usize>,
    ) -> Self {
        Self {
            taker_order_id,
            taker_status,
            taker_side,
            filled_quantity,
            remaining_quantity,
            trades,
            resting_order_idx,
        }
    }
}

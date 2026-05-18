use crate::{
    structure::Trade,
    types::{OrderId, Quantity, State},
};

// this is the report that user will see about their order
pub struct MatchReport {
    pub taker_order_id: OrderId,
    pub taker_status: State,
    pub filled_quantity: Quantity,
    pub remaining_quantity: Quantity,
    pub trades: Vec<Trade>,
}

impl MatchReport {
    pub fn new(
        taker_order_id: OrderId,
        taker_status: State,
        filled_quantity: Quantity,
        remaining_quantity: Quantity,
        trades: Vec<Trade>,
    ) -> Self {
        Self {
            taker_order_id,
            taker_status,
            filled_quantity,
            remaining_quantity,
            trades,
        }
    }
}

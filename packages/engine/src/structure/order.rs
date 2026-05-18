use crate::types::{OrderId, Quantity, Sequence, Side, State, Tick};

pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub tick: Tick,
    pub quantity: Quantity,
    pub remaining_quantity: Quantity,
    pub sequence: Sequence,
    pub status: State,
    pub prev_order: Option<usize>,
    pub next_order: Option<usize>,
}

impl Order {
    pub fn new(
        id: OrderId,
        side: Side,
        tick: Tick,
        quantity: Quantity,
        sequence: Sequence,
    ) -> Self {
        Order {
            id,
            side,
            tick,
            quantity,
            remaining_quantity: quantity,
            sequence,
            status: State::New,
            prev_order: None,
            next_order: None,
        }
    }
}

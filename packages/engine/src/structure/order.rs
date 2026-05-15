pub struct Order {
    pub id: u128,
    pub side: Side,
    pub price: u128,
    pub quantity: u128,
    pub remaining_quantity: u128,
    pub sequence: u128,
    pub status: State,
    pub prev_order_ref: u128,
    pub next_order_ref: u128, // references the position in queue to cancel an order at O(1)
    pub price_ref: u128,      // references the price the order was affecting
}

pub enum Side {
    Ask,
    Bid,
}

pub enum State {
    New,
    Resting,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

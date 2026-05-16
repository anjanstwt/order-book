pub struct Order {
    pub id: u32,
    pub side: Side,
    pub price: u32,
    pub quantity: u32,
    pub remaining_quantity: u32,
    pub sequence: u64,
    pub status: State,
    pub prev_order: Option<usize>,
    pub next_order: Option<usize>,
    pub level_index: Option<usize>,
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

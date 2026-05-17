pub type OrderId = u64;
pub type Tick = u64;
pub type Quantity = u64;
pub type Sequence = u64;

pub enum Side {
    Ask,
    Bid,
}

#[derive(Debug, PartialEq)]
pub enum State {
    New,
    Resting,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

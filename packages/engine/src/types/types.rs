use serde::Deserialize;

pub type OrderId = u64;
pub type Tick = u64;
pub type Quantity = u64;
pub type Sequence = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Side {
    Ask,
    Bid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    New,
    Resting,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

pub const DEFAULT_CAPACITY_TICKS: u64 = 262_144;

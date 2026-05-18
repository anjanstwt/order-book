use crate::{
    book::{AskBook, BidBook},
    engine::MatchReport,
    storage::Storage,
    structure::Order,
    types::{DEFAULT_CAPACITY_TICKS, OrderId, Quantity, Sequence, Side, State, Tick},
};

pub struct Engine {
    pub bids: BidBook,
    pub asks: AskBook,
    pub storage: Storage<Order>,
    pub next_sequence: Sequence,
}

impl Engine {
    pub fn new(capacity_ticks: Option<u64>) -> Self {
        let capacity = capacity_ticks.unwrap_or(DEFAULT_CAPACITY_TICKS);
        Self {
            bids: BidBook::new(Some(capacity)),
            asks: AskBook::new(Some(capacity)),
            storage: Storage::<Order>::new(),
            next_sequence: 0,
        }
    }

    pub fn submit_limit_order(
        &mut self,
        id: OrderId,
        side: Side,
        tick: Tick,
        quantity: Quantity,
    ) -> Result<MatchReport, String> {
        self.next_sequence += 1;
        let order: Order = Order::new(id, side, tick, quantity, self.next_sequence);

        let order_idx = self.storage.insert_value(order);

        match side {
            Side::Ask => {
                self.asks
                    .add_limit_order(&mut self.storage, tick, order_idx)?;
            }
            Side::Bid => {
                self.asks
                    .add_limit_order(&mut self.storage, tick, order_idx)?;
            }
        }

        let match_report = MatchReport::new(id, State::Resting, quantity, quantity, Vec::new());

        Ok(match_report)
    }
}

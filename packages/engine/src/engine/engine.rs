use crate::{
    book::{AskBook, BidBook},
    engine::MatchReport,
    storage::Storage,
    structure::{Order, Trade},
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
            next_sequence: 1,
        }
    }

    pub fn submit_limit_order(
        &mut self,
        id: OrderId,
        side: Side,
        tick: Tick,
        quantity: Quantity,
    ) -> Result<MatchReport, String> {
        let mut remaining = quantity;
        let mut trades = Vec::<Trade>::new();

        // matching against other side of the book
        match side {
            Side::Ask => {
                while remaining > 0 {
                    let Some(best_bid) = self.bids.best_tick else {
                        break;
                    };
                    if best_bid < tick {
                        break;
                    }

                    self.bids.match_at(
                        &mut self.storage,
                        best_bid,
                        id,
                        &mut remaining,
                        &mut self.next_sequence,
                        &mut trades,
                    );
                }
            }
            Side::Bid => {
                while remaining > 0 {
                    let Some(best_ask) = self.asks.best_tick else {
                        break;
                    };
                    if best_ask > tick {
                        break;
                    }

                    self.asks.match_at(
                        &mut self.storage,
                        best_ask,
                        id,
                        &mut remaining,
                        &mut self.next_sequence,
                        &mut trades,
                    );
                }
            }
        }

        let filled = quantity - remaining;
        let taker_status;
        let mut resting_order_idx: Option<usize> = None;
        // put the left over of the order to the limit order
        if remaining > 0 {
            let mut order = Order::new(id, side, tick, quantity, self.next_sequence);
            order.remaining_quantity = remaining;
            self.next_sequence += 1;

            let order_idx = self.storage.insert_value(order);
            resting_order_idx = Some(order_idx);

            match side {
                Side::Ask => self
                    .asks
                    .add_limit_order(&mut self.storage, tick, order_idx)?,
                Side::Bid => self
                    .bids
                    .add_limit_order(&mut self.storage, tick, order_idx)?,
            }

            // changing the fill status
            if filled > 0 {
                self.storage
                    .get_mut_value(order_idx)
                    .expect("just inserted order is missing")
                    .status = State::PartiallyFilled;
                taker_status = State::PartiallyFilled;
            } else {
                taker_status = State::Resting;
            }
        } else {
            taker_status = State::Filled;
        }

        Ok(MatchReport::new(
            id,
            taker_status,
            filled,
            remaining,
            trades,
            resting_order_idx,
        ))
    }

    pub fn submit_market_order(
        &mut self,
        id: OrderId,
        side: Side,
        quantity: Quantity,
    ) -> Result<MatchReport, String> {
        let mut remaining = quantity;
        let mut trades = Vec::<Trade>::new();

        match side {
            Side::Ask => {
                while remaining > 0 {
                    let Some(best_bid) = self.bids.best_tick else {
                        break;
                    };
                    self.bids.match_at(
                        &mut self.storage,
                        best_bid,
                        id,
                        &mut remaining,
                        &mut self.next_sequence,
                        &mut trades,
                    );
                }
            }
            Side::Bid => {
                while remaining > 0 {
                    let Some(best_ask) = self.asks.best_tick else {
                        break;
                    };
                    self.asks.match_at(
                        &mut self.storage,
                        best_ask,
                        id,
                        &mut remaining,
                        &mut self.next_sequence,
                        &mut trades,
                    );
                }
            }
        }

        let filled = quantity - remaining;
        let taker_status: State;

        if remaining == 0 {
            taker_status = State::Filled;
        } else if filled > 0 {
            taker_status = State::PartiallyFilled;
        } else {
            taker_status = State::Canceled;
        }

        Ok(MatchReport::new(
            id,
            taker_status,
            filled,
            remaining,
            trades,
            None,
        ))
    }

    pub fn cancel_order(&mut self, order_idx: usize) -> Result<(), String> {
        let (side, tick) = {
            let order = self
                .storage
                .get_value(order_idx)
                .ok_or_else(|| "order not found".to_string())?;
            (order.side, order.tick)
        };

        match side {
            Side::Ask => {
                self.asks
                    .remove_limit_order(&mut self.storage, tick, order_idx)?;
            }
            Side::Bid => {
                self.bids
                    .remove_limit_order(&mut self.storage, tick, order_idx)?;
            }
        }

        self.storage
            .get_mut_value(order_idx)
            .expect("order vanished mid cancel")
            .status = State::Canceled;

        self.storage.remove_value(order_idx);

        Ok(())
    }
}

use crate::{
    storage::Storage,
    structure::{BitMap, Order, PriceLevel, Trade},
    types::{DEFAULT_CAPACITY_TICKS, OrderId, Quantity, Sequence, Side, State, Tick},
};

pub struct AskBook {
    pub best_tick: Option<Tick>,
    pub levels: Vec<Option<PriceLevel>>,
    bitmap: BitMap,
}

impl AskBook {
    pub fn new(capacity_ticks: Option<u64>) -> Self {
        let capacity = capacity_ticks.unwrap_or(DEFAULT_CAPACITY_TICKS);
        let mut new_levels = Vec::new();
        new_levels.resize_with(capacity as usize, || None);
        Self {
            best_tick: None,
            levels: new_levels,
            bitmap: BitMap::new(Some(capacity)),
        }
    }

    pub fn add_limit_order(
        &mut self,
        storage: &mut Storage<Order>,
        tick: Tick,
        order_idx: usize,
    ) -> Result<(), String> {
        if let Some(price_level) = self.levels.get_mut(tick as usize) {
            if let Some(level) = price_level.as_mut() {
                level.append(storage, order_idx);
                return Ok(());
            }
        }

        let mut price_level = PriceLevel::new(tick);
        price_level.append(storage, order_idx);

        // check if that tick idx available in array
        if self.levels.len() > tick as usize {
            self.levels[tick as usize] = Some(price_level);
            self.bitmap.set(tick);
        } else {
            return Err("Invalid price, it's long..".to_string());
        }

        let Some(best_tick) = self.best_tick else {
            self.best_tick = Some(tick);
            return Ok(());
        };

        if tick < best_tick {
            self.best_tick = Some(tick);
        }
        Ok(())
    }

    pub fn remove_limit_order(
        &mut self,
        storage: &mut Storage<Order>,
        tick: Tick,
        order_idx: usize,
    ) -> Result<Quantity, String> {
        let price_level = self
            .levels
            .get_mut(tick as usize)
            .and_then(|level| level.as_mut())
            .ok_or("The price doesn't exist".to_string())?;

        let quantity = price_level.remove(storage, order_idx);
        if price_level.order_count == 0 {
            self.levels[tick as usize] = None;
            self.bitmap.clear(tick);
        }

        let Some(best_tick) = self.best_tick else {
            return Err("best price not found, panic!".to_string());
        };

        // means this level was the best price
        if best_tick == tick {
            self.best_tick = self.bitmap.first_lowest_tick();
        }

        Ok(quantity)
    }

    pub fn match_at(
        &mut self,
        storage: &mut Storage<Order>,
        tick: Tick,
        taker_id: OrderId,
        taker_quantity: &mut Quantity,
        sequence_counter: &mut Sequence,
        trades: &mut Vec<Trade>,
    ) {
        while *taker_quantity > 0 {
            // re-fetch level everytime
            let Some(level) = self
                .levels
                .get_mut(tick as usize)
                .and_then(|level| level.as_mut())
            else {
                return;
            };

            let Some(head_idx) = level.head_order else {
                break;
            };

            // take maker info and drop borrowing storage, cause we have to borrow it again
            let (maker_remaining, maker_id) = {
                let maker = storage.get_value(head_idx).expect("maker is missing");
                (maker.remaining_quantity, maker.id)
            };

            let fill_quantity = (*taker_quantity).min(maker_remaining);
            // this contains whether the maker is completely filled or not
            let maker_done = fill_quantity == maker_remaining;

            trades.push(Trade::new(
                *sequence_counter,
                maker_id,
                taker_id,
                Side::Ask,
                tick,
                fill_quantity,
            ));

            // changing the sequence, and decreasing the taker's quantity
            *sequence_counter += 1;
            *taker_quantity -= fill_quantity;

            if maker_done {
                // maker is filled completely
                // clean the head of the level
                level.remove(storage, head_idx);
                // change the status of the order
                storage
                    .get_mut_value(head_idx)
                    .expect("maker missing")
                    .status = State::Filled;
                // clean the order from storage
                storage.remove_value(head_idx);
            } else {
                // maker is partially filled
                let maker = storage.get_mut_value(head_idx).expect("maker missing");
                maker.remaining_quantity -= fill_quantity;
                maker.status = State::PartiallyFilled;
                level.total_volume -= fill_quantity;
                break;
            }
        }

        // if the level became empty remove it from the book
        let level_empty = self
            .levels
            .get(tick as usize)
            .and_then(|level| level.as_ref())
            .map(|l| l.order_count == 0)
            .unwrap_or(false);

        if level_empty {
            self.levels[tick as usize] = None;
            self.bitmap.clear(tick);
            if self.best_tick == Some(tick) {
                self.best_tick = self.bitmap.first_lowest_tick();
            }
        }
    }
}

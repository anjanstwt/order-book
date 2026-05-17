use crate::{
    storage::Storage,
    structure::{Order, PriceLevel, Side, Tick},
};

pub struct Book {
    side: Side,
    pub best_tick: Option<Tick>,
    pub levels: Vec<Option<PriceLevel>>,
}

impl Book {
    pub fn new(side: Side) -> Self {
        let mut new_levels = Vec::new();
        new_levels.resize_with(10_000_000, || None);
        Book {
            side,
            best_tick: None,
            levels: new_levels,
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
        } else {
            return Err("Invalid price, it's long..".to_string());
        }

        let Some(best_tick) = self.best_tick else {
            self.best_tick = Some(tick);
            return Ok(());
        };

        match self.side {
            Side::Ask => {
                if tick < best_tick {
                    self.best_tick = Some(tick);
                }
            }
            Side::Bid => {
                if tick > best_tick {
                    self.best_tick = Some(tick);
                }
            }
        }
        Ok(())
    }

    pub fn remove_limit_order(
        &mut self,
        storage: &mut Storage<Order>,
        tick: Tick,
        order_idx: usize,
    ) -> Result<(), String> {
        let price_level = self
            .levels
            .get_mut(tick as usize)
            .and_then(|level| level.as_mut())
            .ok_or("The price doesn't exist".to_string())?;

        price_level.remove(storage, order_idx)?;
        if price_level.order_count == 0 {
            self.levels[tick as usize] = None;
        }

        let Some(best_tick) = self.best_tick else {
            return Err("best price not found, panic!".to_string());
        };

        // means this level was the best price
        if best_tick == tick {
            self.traverse_to_next_best_price(best_tick as usize);
        }

        Ok(())
    }

    fn traverse_to_next_best_price(&mut self, tick_idx: usize) {
        match self.side {
            Side::Ask => {
                let mut i = tick_idx + 1;
                while i < self.levels.len() {
                    if let Some(Some(_val)) = self.levels.get(i) {
                        self.best_tick = Some(i as Tick);
                        return;
                    }

                    i += 1;
                }
                // if this hits means, no asks left
                self.best_tick = None;
            }
            Side::Bid => {
                let mut i = tick_idx as i64 - 1;
                while i >= 0 {
                    if let Some(Some(_val)) = self.levels.get(i as usize) {
                        self.best_tick = Some(i as Tick);
                        return;
                    }
                    i -= 1;
                }
                // if this hits means, no bids left
                self.best_tick = None;
            }
        }
    }
}

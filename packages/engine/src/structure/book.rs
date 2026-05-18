use crate::{
    storage::Storage,
    structure::{BitMap, Order, PriceLevel, Side, Tick},
};

pub struct Book {
    side: Side,
    pub best_tick: Option<Tick>,
    pub levels: Vec<Option<PriceLevel>>,
    bitmap: BitMap,
}

impl Book {
    pub fn new(side: Side, capacity_ticks: u64) -> Self {
        let mut new_levels = Vec::new();
        new_levels.resize_with(capacity_ticks as usize, || None);
        Book {
            side,
            best_tick: None,
            levels: new_levels,
            bitmap: BitMap::new(Some(capacity_ticks)),
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

        price_level.remove(storage, order_idx);
        if price_level.order_count == 0 {
            self.levels[tick as usize] = None;
            self.bitmap.clear(tick);
        }

        let Some(best_tick) = self.best_tick else {
            return Err("best price not found, panic!".to_string());
        };

        // means this level was the best price
        if best_tick == tick {
            self.best_tick = self.find_best();
        }

        Ok(())
    }

    fn find_best(&self) -> Option<Tick> {
        match self.side {
            Side::Ask => self.bitmap.first_lowest_tick(),
            Side::Bid => self.bitmap.first_highest_tick(),
        }
    }
}

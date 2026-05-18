use crate::{
    storage::Storage,
    structure::{BitMap, Order, PriceLevel, Quantity, Tick},
};

pub struct BidBook {
    pub best_tick: Option<Tick>,
    pub levels: Vec<Option<PriceLevel>>,
    bitmap: BitMap,
}

impl BidBook {
    pub fn new(capacity_ticks: Option<u64>) -> Self {
        let capacity = capacity_ticks.unwrap_or(262_144);
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

        if tick > best_tick {
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
            self.best_tick = self.bitmap.first_highest_tick();
        }

        Ok(quantity)
    }
}

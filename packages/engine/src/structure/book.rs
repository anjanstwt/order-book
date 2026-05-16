use std::collections::HashMap;

use crate::{
    storage::Storage,
    structure::{Order, PriceLevel, Side},
};

pub struct Book {
    side: Side,
    pub best_price: u32,
    pub levels: HashMap<u32, PriceLevel>,
}

impl Book {
    pub fn new(side: Side) -> Self {
        Book {
            side,
            best_price: 0,
            levels: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, storage: &mut Storage<Order>, price: u32, order_idx: usize) {
        if let Some(price_level) = self.levels.get_mut(&price) {
            price_level.append(storage, order_idx);
            return;
        }
        let mut price_level = PriceLevel::new(price);
        price_level.append(storage, order_idx);
        self.levels.insert(price, price_level);

        // update the best price
        match self.side {
            Side::Ask => {
                if price < self.best_price {
                    self.best_price = price;
                }
            }
            Side::Bid => {
                if price > self.best_price {
                    self.best_price = price;
                }
            }
        }
    }

    pub fn remove_order(
        &mut self,
        storage: &mut Storage<Order>,
        price: u32,
        order_idx: usize,
    ) -> Result<(), String> {
        let should_remove;

        {
            let Some(price_level) = self.levels.get_mut(&price) else {
                return Err("no price level found".to_string());
            };

            price_level.remove(storage, order_idx)?;

            should_remove = price_level.order_count == 0;
        }

        if should_remove {
            self.levels.remove(&price);
        }

        Ok(())
    }
}

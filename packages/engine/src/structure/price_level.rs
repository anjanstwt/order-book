use crate::{storage::Storage, structure::Order};

pub struct PriceLevel {
    pub price: u32,
    pub total_volume: u32,
    pub order_count: u32,
    pub head_order: Option<usize>,
    pub tail_order: Option<usize>,
}

impl PriceLevel {
    pub fn new(price: u32) -> Self {
        PriceLevel {
            price,
            total_volume: 0,
            order_count: 0,
            head_order: None,
            tail_order: None,
        }
    }

    pub fn append(&mut self, storage: &mut Storage<Order>, order_idx: usize) {
        // check if the price level is empty for this price..
        if self.head_order.is_none() {
            // get the current order from storage
            let Some(order) = storage.get_mut_value(order_idx) else {
                return;
            };

            self.head_order = Some(order_idx);
            self.tail_order = Some(order_idx);
            self.order_count += 1;
            self.total_volume += order.remaining_quantity;
            order.prev_order = None;
            order.next_order = None;
            return;
        }

        // this will never hit as it is checked before
        let Some(tail) = self.tail_order else {
            return;
        };

        // doing it in a different block to avoid conflict of getting mutable orders..
        {
            // get the current order from storage
            let Some(order) = storage.get_mut_value(order_idx) else {
                return;
            };

            // update the new order with next and prev
            order.next_order = None;
            order.prev_order = Some(tail);

            // increase the total volume and order count
            self.total_volume += order.remaining_quantity;
            self.order_count += 1;
        }
        // point the tail order to the new order index
        self.tail_order = Some(order_idx);

        // get the prev order
        let Some(prev_order) = storage.get_mut_value(tail) else {
            return;
        };

        // update the next order index to the new order index
        prev_order.next_order = Some(order_idx);
    }

    pub fn remove(&mut self, storage: &mut Storage<Order>, order_idx: usize) -> Result<(), String> {
        // get the indexes and remaining quantity
        let (prev_idx, next_idx, remaining_quantity) = {
            // get the current order
            let Some(order) = storage.get_value(order_idx) else {
                return Err("no order found".to_string());
            };
            (order.prev_order, order.next_order, order.remaining_quantity)
        };

        // check if prev is present, if not means it is the head order
        if let Some(prev) = prev_idx {
            let Some(prev_order) = storage.get_mut_value(prev) else {
                return Err("no previous order exists".to_string());
            };

            prev_order.next_order = next_idx;
        } else {
            self.head_order = next_idx;
        }

        // check if next is present, if not means it is the tail order
        if let Some(next) = next_idx {
            let Some(next_order) = storage.get_mut_value(next) else {
                return Err("no next orders exists".to_string());
            };

            next_order.prev_order = prev_idx;
        } else {
            self.tail_order = prev_idx;
        }

        self.total_volume -= remaining_quantity;
        self.order_count -= 1;

        if let Some(order) = storage.get_mut_value(order_idx) {
            order.prev_order = None;
            order.next_order = None;
        }

        Ok(())
    }

    pub fn has_no_orders(&self) -> bool {
        if self.order_count == 0 {
            return true;
        }
        false
    }
}

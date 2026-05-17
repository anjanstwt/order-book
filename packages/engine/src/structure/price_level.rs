use crate::{
    storage::Storage,
    structure::{Order, Quantity, State, Tick},
};

pub struct PriceLevel {
    pub tick: Tick,
    pub total_volume: Quantity,
    pub order_count: u32,
    pub head_order: Option<usize>,
    pub tail_order: Option<usize>,
}

impl PriceLevel {
    pub fn new(tick: Tick) -> Self {
        PriceLevel {
            tick,
            total_volume: 0,
            order_count: 0,
            head_order: None,
            tail_order: None,
        }
    }

    pub fn append(&mut self, storage: &mut Storage<Order>, order_idx: usize) {
        // check if the price level is empty for this tick..
        if self.head_order.is_none() {
            // get the current order from storage
            let order = storage
                .get_mut_value(order_idx)
                .expect("order missing in storage");

            assert_eq!(
                order.tick, self.tick,
                "order tick {} and level tick {} are not equal",
                order.tick, self.tick,
            );
            assert_eq!(order.status, State::New, "It's not a new order");
            debug_assert!(order.remaining_quantity > 0, "zero quantity found");

            self.head_order = Some(order_idx);
            self.tail_order = Some(order_idx);
            self.order_count += 1;
            self.total_volume += order.remaining_quantity;
            order.prev_order = None;
            order.next_order = None;
            return;
        }

        // this will never hit as it is checked before
        let tail = self
            .tail_order
            .expect("Panic! no tail order found even after head order");

        // doing it in a different block to avoid conflict of getting mutable orders..
        {
            // get the current order from storage
            let order = storage
                .get_mut_value(order_idx)
                .expect("order missing in storage");

            assert_eq!(
                order.tick, self.tick,
                "order tick {} and level tick {} are not equal",
                order.tick, self.tick,
            );
            assert_eq!(order.status, State::New, "It's not a new order");
            debug_assert!(order.remaining_quantity > 0, "zero quantity found");

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
        let prev_order = storage
            .get_mut_value(tail)
            .expect("no prev order found in storage");

        // update the next order index to the new order index
        prev_order.next_order = Some(order_idx);
    }

    pub fn remove(&mut self, storage: &mut Storage<Order>, order_idx: usize) -> Quantity {
        // get the indexes and remaining quantity
        let (prev_idx, next_idx, remaining_quantity) = {
            // get the current order
            let order = storage
                .get_value(order_idx)
                .expect("order missing in storage");

            assert_eq!(
                order.tick, self.tick,
                "order tick {} and level tick {} are not equal",
                order.tick, self.tick,
            );
            assert!(
                matches!(order.status, State::Resting | State::PartiallyFilled),
                "order not live"
            );

            (order.prev_order, order.next_order, order.remaining_quantity)
        };

        // check if prev is present, if not means it is the head order
        if let Some(prev) = prev_idx {
            let prev_order = storage.get_mut_value(prev).expect("no prev order exists");

            prev_order.next_order = next_idx;
        } else {
            self.head_order = next_idx;
        }

        // check if next is present, if not means it is the tail order
        if let Some(next) = next_idx {
            let next_order = storage.get_mut_value(next).expect("no next order exists");

            next_order.prev_order = prev_idx;
        } else {
            self.tail_order = prev_idx;
        }

        self.total_volume -= remaining_quantity;
        self.order_count -= 1;

        let order = storage
            .get_mut_value(order_idx)
            .expect("order not found in storage.");

        assert_eq!(
            order.tick, self.tick,
            "order tick {} and level tick {} are not equal",
            order.tick, self.tick,
        );
        assert!(
            matches!(order.status, State::Resting | State::PartiallyFilled),
            "order not live"
        );

        order.prev_order = None;
        order.next_order = None;

        remaining_quantity
    }
}

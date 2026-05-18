// using the free_slots as LIFO not FIFO as the last putted free slot will more likely to be in
// cache than the prev one
pub struct Storage<T> {
    pub values: Vec<Option<T>>,
    free_slots: Vec<usize>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Storage {
            values: Vec::new(),
            free_slots: Vec::new(),
        }
    }

    pub fn insert_value(&mut self, value: T) -> usize {
        if let Some(idx) = self.free_slots.pop() {
            self.values[idx] = Some(value);
            return idx;
        }

        self.values.push(Some(value));
        return self.values.len() - 1;
    }

    pub fn remove_value(&mut self, idx: usize) {
        debug_assert!(idx < self.values.len(), "index {idx} out of bounds");
        assert!(
            self.values[idx].is_some(),
            "slot index {idx} is already empty",
        );

        self.values[idx] = None;
        self.free_slots.push(idx);
    }

    pub fn get_value(&self, idx: usize) -> Option<&T> {
        self.values.get(idx)?.as_ref()
    }

    pub fn get_mut_value(&mut self, idx: usize) -> Option<&mut T> {
        self.values.get_mut(idx)?.as_mut()
    }
}

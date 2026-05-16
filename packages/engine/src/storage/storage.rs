use std::collections::VecDeque;

pub struct Storage<T> {
    pub values: Vec<Option<T>>,
    free_slots: VecDeque<usize>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Storage {
            values: Vec::new(),
            free_slots: VecDeque::new(),
        }
    }

    pub fn insert_value(&mut self, value: T) -> usize {
        if let Some(idx) = self.free_slots.pop_front() {
            self.values[idx] = Some(value);
            return idx;
        }

        self.values.push(Some(value));
        return self.values.len() - 1;
    }

    pub fn remove_value(&mut self, idx: usize) {
        if idx >= self.values.len() || self.values.get(idx).is_none() {
            return;
        }

        self.values[idx] = None;
        self.free_slots.push_back(idx);
    }

    pub fn get_value(&self, idx: usize) -> Option<&T> {
        self.values.get(idx)?.as_ref()
    }

    pub fn get_mut_value(&mut self, idx: usize) -> Option<&mut T> {
        self.values.get_mut(idx)?.as_mut()
    }
}

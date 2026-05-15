pub struct Storage<T> {
    pub values: Vec<Option<T>>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Storage { values: Vec::new() }
    }

    pub fn insert(&mut self, value: T) -> u128 {
        self.values.push(Some(value));
        return self.values.len() as u128 - 1;
    }

    pub fn remove(&mut self, idx: usize) {
        self.values[idx] = None;
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.values.get(idx)?.as_ref()
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.values.get_mut(idx)?.as_mut()
    }
}

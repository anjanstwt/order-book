use std::collections::VecDeque;

pub struct Queue<T> {
    pub values: VecDeque<Option<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            values: VecDeque::new(),
        }
    }

    pub fn append(&mut self, value: T) {
        self.values.push_back(Some(value));
    }

    pub fn remove(&mut self) {

    }

    pub fn pop_head(&mut self) {

    }

}

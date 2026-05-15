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

    pub fn push(&mut self, value: T) {
        self.values.push_back(Some(value));
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.values.pop_front()?;
    }
}

use std::collections::VecDeque;
use std::ops::Index;

pub struct CycleBuffer<T> {
    data: VecDeque<T>,
    shift: usize,
    capacity: usize,
}

impl<T> CycleBuffer<T> {
    pub fn new(capacity: Option<usize>) -> Self {
        Self {
            data: VecDeque::new(),
            shift: 0,
            capacity: capacity.unwrap_or(u32::MAX as usize),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.data.len() == self.capacity {
            self.shift += 1;
            self.data.pop_front();
        }
        self.data.push_back(item);
    }

    pub fn len(&self) -> usize {
        self.shift + self.data.len()
    }
}

impl<T> Index<usize> for CycleBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index - self.shift]
    }
}
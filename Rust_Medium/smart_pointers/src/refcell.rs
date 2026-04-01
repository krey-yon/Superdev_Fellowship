use std::cell::RefCell;

pub struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

pub fn count_to(n: i32) -> i32 {
    let counter = Counter::new();
    for _ in 0..n {
        counter.increment();
    }
    counter.get()
}

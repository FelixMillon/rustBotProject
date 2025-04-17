use std::sync::{Arc, Mutex};

pub struct IDGenerator {
    counter: u32,
}
impl IDGenerator {
    pub fn new() -> Self {
        IDGenerator { counter: 0 }
    }

    pub fn generate_id(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}
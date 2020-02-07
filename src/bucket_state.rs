use crate::time_window::TimeWindow;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BucketState {
    limit: u32,
    key: String,
    window: TimeWindow,
    count: u32,
    previous_state: Box<Option<BucketState>>
}

impl BucketState {
    pub fn new(key: &str, window: &TimeWindow, limit: u32) -> Self {
        Self {
            key: key.to_owned(),
            limit,
            window: window.clone(),
            count: 0,
            previous_state: Box::new(None)
        }
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn get_window(&self) -> &TimeWindow {
        &self.window
    }

    fn next(&self, window: &TimeWindow) -> Self {
        let previous_state = Box::new(if window.is_next(&self.window) {
            Some(self.clone())
        } else {
            None
        });

        Self {
            key: self.key.clone(),
            limit: self.limit,
            window: window.clone(),
            count: 0,
            previous_state
        }
    }

    pub fn increment(&mut self, delta: u32, window: &TimeWindow) -> u32 {
        if self.window != *window && window.is_next(&self.window) {
            if window.is_next(&self.window) {
                let next = self.next(&window);
                self.key = next.key;
                self.limit = next.limit;
                self.window = next.window;
                self.count = next.count;
                self.previous_state = next.previous_state; 
            }

            if !window.is_after_next(&self.window) {
                self.previous_state = Box::new(None);
            }
        }

        self.count += delta;

        self.count
    }
}

use crate::time::TimeWindow;
use std::time::Duration;
use std::time::Instant;
use crate::ratelimiting::RateLimitStrategy;
use chrono::offset::Utc;
use chrono::DateTime;

#[derive(Debug, Clone)]
pub struct BucketState {
    pub limit: u32,
    pub key: String,
    pub window: TimeWindow,
    pub global_count: u32,
    pub local_count: u32,
    pub previous_state: Box<Option<BucketState>>
}

impl BucketState {
    pub fn new(key: &str, window: &TimeWindow, limit: u32) -> Self {
        Self {
            key: key.to_owned(),
            limit,
            window: window.clone(),
            local_count: 0,
            global_count: 0,
            previous_state: Box::new(None)
        }
    }

    pub fn get_count(&self) -> u32 {
        self.global_count + self.local_count
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
            local_count: 0,
            global_count: 0,
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
                self.local_count = next.local_count;
                self.global_count = next.global_count;
                self.previous_state = next.previous_state; 
            }

            if !window.is_after_next(&self.window) {
                self.previous_state = Box::new(None);
            }
        }

        self.local_count += delta;

        self.get_count()
    }

    pub fn clear_local_count(&mut self) -> u32 {
        let count = self.local_count;
        self.global_count += count;

        return count;
    }

    pub fn set_global_count(&mut self, global_count: u32) {
        if global_count > self.global_count {
            self.global_count = global_count;
        }
    }

    pub fn is_rate_limited<S: RateLimitStrategy>(&self, instance: DateTime<Utc>, strat: &S) -> bool {
        strat.is_rate_limited(instance, self, &*self.previous_state)
    }
}

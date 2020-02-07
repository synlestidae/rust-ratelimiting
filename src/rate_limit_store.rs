use chashmap::CHashMap;
use crate::bucket_state::BucketState;
use crate::time_window::TimeWindow;
use std::borrow::Borrow;

pub struct RateLimitStore {
    limit: u32,
    buckets: CHashMap<String, BucketState>
}

impl RateLimitStore {
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            buckets: CHashMap::new()
        }
    }

    pub fn is_rate_limited(&self, key: &str, window: TimeWindow, limit: u32) -> bool {
        match self.buckets.get(key) {
            Some(ref bucket) => bucket.get_window().is_after_next(&window) && bucket.get_count() >= limit,
            None => false
        }
    }

    pub fn increment(&self, key: &str, window: &TimeWindow, change: u32) {
        let mut new_state = BucketState::new(key, window, self.limit);
        new_state.increment(change, window);

        self.buckets.upsert(key.to_owned(), || new_state, |bucket| { bucket.increment(change, window); } );
    }
}

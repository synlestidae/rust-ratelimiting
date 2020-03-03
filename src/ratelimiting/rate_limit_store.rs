use chashmap::CHashMap;
use chrono::DateTime;
use chrono::offset::Utc;
use crate::bucket::BucketState;
use crate::ratelimiting::RateLimitStrategy;
use crate::time::TimeWindow;

#[derive(Clone)]
pub struct RateLimitStore<S: RateLimitStrategy> {
    buckets: CHashMap<String, BucketState>,
    rate_limit_strategy: S
}

impl<S: RateLimitStrategy> RateLimitStore<S> {
    pub fn new(rate_limit_strategy: S) -> Self {
        Self {
            rate_limit_strategy,
            buckets: CHashMap::new()
        }
    }

    pub fn is_rate_limited(&self, key: &str, window: &TimeWindow, instance: &DateTime<Utc>) -> bool {
        match self.buckets.get(key) {
            Some(ref bucket) => bucket.is_rate_limited(instance.clone(), &self.rate_limit_strategy),
            None => false
        }
    }

    pub fn increment(&self, key: &str, window: &TimeWindow, change: u32) {
        let mut new_state = BucketState::new(key, window, self.rate_limit_strategy.limit(key));
        new_state.increment(change, window);

        self.buckets.upsert(key.to_owned(), || new_state, |bucket| { bucket.increment(change, window); } );
    }
}

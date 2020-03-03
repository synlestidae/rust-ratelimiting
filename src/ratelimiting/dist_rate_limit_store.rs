use chashmap::CHashMap;
use chrono::DateTime;
use chrono::offset::Utc;
use crate::periodic::UpdateStrategy;
use crate::periodic::UpdateTracker;
use crate::time::TimeWindow;
use crate::store::GlobalStore;
use crate::ratelimiting::DistBucketState;
use crate::ratelimiting::DistBucketFactory;
use std::ops::DerefMut;

pub struct DistRateLimitStore<F: DistBucketFactory, G: GlobalStore> {
    dist_bucket_factory: F,
    buckets: CHashMap<String, DistBucketState<F::R, F::T, F::S>>,
    global_store: G,
}

impl<F: DistBucketFactory, G: GlobalStore> DistRateLimitStore<F, G> {
    pub fn new(dist_bucket_factory: F, global_store: G) -> Self {
        Self {
            buckets: CHashMap::new(),
            dist_bucket_factory,
            global_store
        }
    }

    pub fn is_rate_limited(&self, key: &str, _window: &TimeWindow, instance: &DateTime<Utc>) -> bool {
        match self.buckets.get(key) {
            Some(ref bucket) => { 
                let rate_limit_strategy = &bucket.rate_limit_strategy;
                bucket.bucket_state.is_rate_limited(instance.clone(), rate_limit_strategy) 
            },
            None => false
        }
    }

    pub fn increment(&mut self, key: &str, window: &TimeWindow, change: u32) {
        let key_copy = key.to_owned();
        let window_copy = window.to_owned();
        //let limit = self.rate_limit_strategy.limit(key);
        // exclusive zone begins here
        let mut factory = self.dist_bucket_factory.clone();

        self.buckets.upsert(key.to_owned(), 
            move || factory.make(&key_copy, &window_copy),
            |bucket| { bucket.bucket_state.increment(change, window); } 
        );

        if let Some(ref mut dist_bucket_write_guard) = self.buckets.get_mut(key) {
            let dist_bucket = dist_bucket_write_guard.deref_mut();

            let needs_update = { 
                let bucket = &dist_bucket.bucket_state;
                let strategy = &mut dist_bucket.update_strategy;
                strategy.needs_update(bucket)
            };

            if needs_update {
                let bucket_state = &mut dist_bucket.bucket_state;
                let tracker = &mut dist_bucket.update_tracker;
                let update = tracker.build_update(bucket_state);

                let key = update.key();
                let increment = update.global_increment();

                self.global_store.increment(&key, increment).unwrap(); // TODO consume value

                let current_value = self.global_store.get(&key).unwrap();

                dist_bucket.bucket_state.set_global_count(current_value);
            }
        }
    }
}

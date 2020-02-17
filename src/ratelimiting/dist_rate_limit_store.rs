use chashmap::CHashMap;
use chrono::DateTime;
use chrono::offset::Utc;
use crate::bucket_state::BucketState;
use crate::rate_limit_strategy::RateLimitStrategy;
use crate::time_window::TimeWindow;
use crate::update_state::UpdateState;
use crate::update_tracker::UpdateTracker;
use redis::Client;
use redis::IntoConnectionInfo;
use redis::RedisError;
use std::borrow::Borrow;

pub struct DistRateLimitStore<S: RateLimitStrategy> {
    buckets: CHashMap<String, DistBucketState>,
    rate_limit_strategy: S,
    redis_uri: String
}

impl<S: RateLimitStrategy> DistRateLimitStore<S> {
    pub fn new(redis_uri: &str, rate_limit_strategy: S) -> Self {
        Self {
            buckets: CHashMap::new(),
            rate_limit_strategy,
            redis_uri: redis_uri.to_owned()
        }
    }

    pub fn is_rate_limited(&self, key: &str, window: &TimeWindow, instance: &DateTime<Utc>) -> bool {
        match self.buckets.get(key) {
            Some(ref bucket) => bucket.bucket_state.is_rate_limited(instance.clone(), &self.rate_limit_strategy),
            None => false
        }
    }

    pub fn increment(&mut self, key: &str, window: &TimeWindow, change: u32) {
        println!("Begin increment");
        let mut bucket_state = BucketState::new(key, window, self.rate_limit_strategy.limit(key));
        bucket_state.increment(change, window);
        let new_dist_bucket_state = DistBucketState {
            bucket_state,
            update_tracker: UpdateTracker::new(0, self.rate_limit_strategy.limit(key) / 20)
        };

        // exclusive zone begins here

        self.buckets.upsert(key.to_owned(), || new_dist_bucket_state, |bucket| { bucket.bucket_state.increment(change, window); } );

        let update_state_option = if let Some(ref mut dist_bucket_write_guard) = self.buckets.get_mut(key) {
            if let Some(current_global_value) = dist_bucket_write_guard.update_tracker.refresh() {
                dist_bucket_write_guard.bucket_state.set_global_count(current_global_value);
            };
            let needs_update = { 
                dist_bucket_write_guard.update_tracker.needs_update(&dist_bucket_write_guard.bucket_state)
            };

            let mut bucket_state = dist_bucket_write_guard.bucket_state.clone();

            let dbwgo = if needs_update {
                Some(dist_bucket_write_guard.update_tracker.prep_update(&mut bucket_state))
            } else {
                None
            };

            drop(dist_bucket_write_guard);

            dbwgo
        } else {
            None
        };

        // exclusive zone ends
        if let Some(mut update_state) = update_state_option {
            println!("update_state: {:?}", update_state);
            let new_val = Self::global_increment(&self.redis_uri, &mut update_state).unwrap();
            update_state.read_success(new_val);
            println!("All done!");
        }

        println!("Finish increment");
    }

    fn global_increment(redis_uri: &str, update_state: &mut UpdateState) -> Result<u32, RedisError> {
        let mut connection = Client::open(redis_uri)?;

        let key = match update_state.key() {
            Some(k) => k,
            None => return Ok(0) // TODO BAD BAD BAD BAD BAD
        };

        let increment_command = redis::cmd("INCRBY")
            .arg(&key)
            .arg(&update_state.global_increment().to_string())
            .query(&mut connection)?;

        redis::cmd("GET")
            .arg(&key)
            .query::<u32>(&mut connection)
    }
}

struct DistBucketState {
    bucket_state: BucketState,
    update_tracker: UpdateTracker
}

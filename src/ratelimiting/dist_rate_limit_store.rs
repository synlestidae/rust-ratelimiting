use chashmap::CHashMap;
use chrono::DateTime;
use chrono::offset::Utc;
use crate::bucket::BucketState;
use crate::periodic::PeriodicUpdateTracker;
use crate::periodic::UpdateStrategy;
use crate::periodic::PeriodicUpdateStrategy;
use crate::periodic::UpdateState;
use crate::periodic::UpdateValue;
use crate::periodic::UpdateTracker;
use crate::ratelimiting::RateLimitStrategy;
use crate::time::TimeWindow;
use redis::Client;
use redis::IntoConnectionInfo;
use redis::RedisError;
use std::borrow::Borrow;

pub struct DistRateLimitStore<R: RateLimitStrategy, T: UpdateTracker, S: UpdateStrategy> {
    buckets: CHashMap<String, DistBucketState<T, S>>,
    rate_limit_strategy: R,
    redis_uri: String
}

impl<R: RateLimitStrategy, T: UpdateTracker, S: UpdateStrategy> DistRateLimitStore<R, T, S> {
    pub fn new(redis_uri: &str, rate_limit_strategy: R) -> Self {
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
        let mut bucket_state = BucketState::new(key, window, self.rate_limit_strategy.limit(key));
        let update_tracker = T::from(&bucket_state);
        let update_strategy = S::from(&bucket_state);
        bucket_state.increment(change, window);

        let new_dist_bucket_state = DistBucketState {
            bucket_state,
            update_tracker,
            update_strategy,
            state: None
        };

        // exclusive zone begins here

        self.buckets.upsert(key.to_owned(), || new_dist_bucket_state, |bucket| { bucket.bucket_state.increment(change, window); } );

        let update_state_option = if let Some(ref mut dist_bucket_write_guard) = self.buckets.get_mut(key) {
            if let Some(current) = dist_bucket_write_guard.poll_update() {
                dist_bucket_write_guard.bucket_state.set_global_count(current.global_value);
            };
            let needs_update = !dist_bucket_write_guard.state.is_some() && { 
                let bucket = dist_bucket_write_guard.bucket_state.clone();
                let strategy = &mut dist_bucket_write_guard.update_strategy;
                strategy.needs_update(&bucket)
            };

            let mut bucket_state = dist_bucket_write_guard.bucket_state.clone();

            let dbwgo = if needs_update {
                Some(dist_bucket_write_guard.update_tracker.build_update(&mut bucket_state))
            } else {
                None
            };

            dist_bucket_write_guard.state = dbwgo.clone();

            drop(dist_bucket_write_guard);

            dbwgo
        } else {
            None
        };

        // exclusive zone ends
        if let Some(mut update_state) = update_state_option {
            let new_val = Self::global_increment(&self.redis_uri, &mut update_state).unwrap();

            update_state.read_success(new_val);
        }
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

struct DistBucketState<U: UpdateTracker, S: UpdateStrategy> {
    bucket_state: BucketState,
    update_tracker: U,
    update_strategy: S,
    state: Option<UpdateState>
}

impl<U: UpdateTracker, S: UpdateStrategy> DistBucketState<U, S> {
    fn poll_update(&mut self) -> Option<UpdateValue> {
        let mut is_done = false;
        let mut return_value: Option<u32> = None;

        if let Some(ref mut s) = &mut self.state {
            is_done = s.is_done();

            if is_done && !s.is_failed() {
                return_value = Some(s.global_value());
            }
        }

        if is_done {
            self.state = None;
        }

        return_value.map(|v| UpdateValue::new(&self.bucket_state.key, v))
    }
}


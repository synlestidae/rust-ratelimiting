use crate::ratelimiting::DistRateLimitStore;
use crate::ratelimiting::SlidingWindowRateLimitStrategy;
use crate::periodic::PeriodicUpdateTracker;
use crate::periodic::PeriodicUpdateStrategy;
use chrono::offset::Utc;
use chrono::Duration;
use crate::time::TimeWindow;
use std::thread::spawn;
use std::{thread, time as std_time};
use crate::ratelimiting::DistBucketFactory;
use crate::redis::IntoConnectionInfo;
use crate::ratelimiting::*;
use crate::bucket::*;
use crate::periodic::*;
use crate::store::RedisStore;

#[derive(Clone)]
struct TestDistBucketFactory; 

impl DistBucketFactory for TestDistBucketFactory {
    type R = SlidingWindowRateLimitStrategy;
    type T = PeriodicUpdateTracker; 
    type S = PeriodicUpdateStrategy;

    fn make(&mut self, key: &str, window: &TimeWindow) -> DistBucketState<Self::R, Self::T, Self::S> {
        let bucket_state = BucketState::new(key, window, 2000);

        let rate_limit_strategy = SlidingWindowRateLimitStrategy::new(2000, 10);
        let update_tracker = UpdateTracker::from(&bucket_state);
        let update_strategy = UpdateStrategy::from(&bucket_state);

        DistBucketState {
            bucket_state,
            update_tracker,
            update_strategy,
            rate_limit_strategy,
            state: None
        }
    }
}

#[test]
pub fn increments_value_in_rediss() {
    let mut thread_things = Vec::new();

    for _ in 0..5 {
        thread_things.push(spawn(|| {
            let redis_store = RedisStore::new("redis://127.0.0.1/".into_connection_info().unwrap());
            let mut store: DistRateLimitStore<TestDistBucketFactory, RedisStore> = 
                DistRateLimitStore::new(TestDistBucketFactory, redis_store);

            for _ in 0..3000 {
                let one_milli = std_time::Duration::from_millis(10 * 5);

                thread::sleep(one_milli);

                if !store.is_rate_limited("test", &TimeWindow::from(Utc::now(), Duration::minutes(5)), &Utc::now()) {
                    store.increment("test", &TimeWindow::from(Utc::now(), Duration::minutes(5)), 1);
                }
            }
        }));
    }

    for t in thread_things.into_iter() {
        t.join().unwrap();
    }
}

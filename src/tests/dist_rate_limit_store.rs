use crate::ratelimiting::DistRateLimitStore;
use crate::ratelimiting::SlidingWindowRateLimitStrategy;
use crate::periodic::PeriodicUpdateTracker;
use crate::periodic::RandomUpdateStrategy;
use chrono::offset::Utc;
use chrono::Duration;
use crate::time::TimeWindow;
use std::thread::sleep;
use std::thread::spawn;
use std::{thread, time as std_time};

#[test]
pub fn increments_value_in_rediss() {
    let mut thread_things = Vec::new();

    for i in 0..5 {
        thread_things.push(spawn(|| {
            let mut store: DistRateLimitStore<SlidingWindowRateLimitStrategy, PeriodicUpdateTracker, RandomUpdateStrategy> = 
                DistRateLimitStore::new("redis://127.0.0.1/", SlidingWindowRateLimitStrategy::new(600, 4));

            for i in 0..300 {
                let one_milli = std_time::Duration::from_millis(1000 * 5);

                thread::sleep(one_milli);

                if !store.is_rate_limited("test", &TimeWindow::from(Utc::now(), Duration::minutes(5)), &Utc::now()) {
                    store.increment("test", &TimeWindow::from(Utc::now(), Duration::minutes(5)), 1);
                }
            }
        }));
    }

    for t in thread_things.into_iter() {
        t.join();
    }
}

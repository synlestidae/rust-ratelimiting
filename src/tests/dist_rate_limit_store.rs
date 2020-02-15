use crate::dist_rate_limit_store::DistRateLimitStore;
use crate::sliding_window_rate_limit_strategy::SlidingWindowRateLimitStrategy;
use chrono::offset::Utc;
use chrono::Duration;
use crate::time_window::TimeWindow;
use std::thread::sleep;
use std::thread::spawn;
use std::{thread, time as std_time};

#[test]
pub fn increments_value_in_rediss() {
    let mut thread_things = Vec::new();

    for i in 0..6 {
        thread_things.push(spawn(|| {
            let mut store = DistRateLimitStore::new("redis://127.0.0.1/", SlidingWindowRateLimitStrategy::new(36000));

            for i in 0..300 {
                let one_milli = std_time::Duration::from_millis(10);

                thread::sleep(one_milli);

                if true || !store.is_rate_limited("test", &TimeWindow::from(Utc::now(), Duration::minutes(5)), &Utc::now()) {
                    store.increment("test", &TimeWindow::from(Utc::now(), Duration::minutes(5)), 20);
                }

            }
        }));
    }

    for t in thread_things.into_iter() {
        t.join();
    }
}

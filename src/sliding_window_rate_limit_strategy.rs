use crate::bucket_state::BucketState;
use crate::rate_limit_strategy::RateLimitStrategy;
use chrono::Duration;
use crate::time_window::TimeWindow;
use chrono::DateTime;
use chrono::offset::Utc;

/// I'm from a Java background
pub struct SlidingWindowRateLimitStrategy {}

impl RateLimitStrategy for SlidingWindowRateLimitStrategy  {
    fn is_rate_limited(&self, instance: DateTime<Utc>, current: &BucketState, previous: &Option<BucketState>) -> bool {
        let current_limit = f64::from(current.limit);
        let current_value = f64::from(current.count);

        let slide_value: f64 = match previous {
            Some(ref previous) => current_limit * previous.window.slide_ratio(&instance),
            None => 0.0
        };

        println!("RateLimitStrategy: {} {}", current_value, slide_value);

        current_value + slide_value > current_limit
    }
}

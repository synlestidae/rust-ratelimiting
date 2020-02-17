use chrono::DateTime;
use chrono::Duration;
use chrono::offset::Utc;
use crate::bucket::BucketState;
use crate::ratelimiting::RateLimitStrategy;
use crate::time::TimeWindow;

#[derive(Clone)]
pub struct SlidingWindowRateLimitStrategy {
    default_limit: u32,
    approx_node_count: u32
}

impl SlidingWindowRateLimitStrategy {
    pub fn new(default_limit: u32, approx_node_count: u32) -> Self {
        Self {
            default_limit,
            approx_node_count
        }
    }
}

impl RateLimitStrategy for SlidingWindowRateLimitStrategy  {
    fn is_rate_limited(&self, instance: DateTime<Utc>, current: &BucketState, previous: &Option<BucketState>) -> bool {
        let current_limit = f64::from(current.limit);
        let current_value = f64::from(current.get_count());
        let slide_ratio = current.window.slide_ratio(&instance);

        let slide_value: f64 = match previous {
            Some(ref previous) => if slide_ratio < 1.0 { f64::from(previous.get_count()) * (1.0 - slide_ratio) } else { 0.0 },
            None => 0.0
        };

        println!("rate limit = {} + {} >= {}", current_value, slide_value, current_limit);

        current_value + slide_value >= current_limit
    }

    //fn update_threshold_hint(&self, instance: DateTime<Utc>, bucket: &BucketState) -> Option<u32> {
    //    Some(self.approx_node_count)
    //}

    fn limit(&self, key: &str) -> u32 {
        self.default_limit
    }
}

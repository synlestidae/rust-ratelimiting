use chrono::DateTime;
use chrono::offset::Utc;
use crate::bucket::BucketState;
use crate::ratelimiting::RateLimitStrategy;

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
        self.get_count(instance, current, previous) >= current.limit
    }

    fn get_count(&self, instance: DateTime<Utc>, current: &BucketState, previous: &Option<BucketState>) -> u32 {
        let current_value = f64::from(current.get_count());
        let slide_ratio = current.window.slide_ratio(&instance);

        let slide_value: f64 = match previous {
            Some(ref previous) => if slide_ratio < 1.0 { f64::from(previous.get_count()) * (1.0 - slide_ratio) } else { 0.0 },
            None => 0.0
        };

        (current_value + slide_value).floor() as u32
    }

    fn limit(&self, _key: &str) -> u32 {
        self.default_limit
    }
}

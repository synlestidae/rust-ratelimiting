use crate::bucket_state::BucketState;
use crate::time_window::TimeWindow;
use chrono::DateTime;
use chrono::offset::Utc;

pub trait RateLimitStrategy: Clone {
    fn is_rate_limited(&self, instance: DateTime<Utc>, current: &BucketState, previous: &Option<BucketState>) -> bool;

    fn get_count(&self, instance: DateTime<Utc>, current: &BucketState, previous: &Option<BucketState>) -> u32;

    fn limit(&self, key: &str) -> u32;
}

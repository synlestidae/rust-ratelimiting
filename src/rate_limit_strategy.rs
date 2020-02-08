use crate::bucket_state::BucketState;
use crate::time_window::TimeWindow;
use chrono::DateTime;
use chrono::offset::Utc;

pub trait RateLimitStrategy {
    fn is_rate_limited(&self, instance: DateTime<Utc>, current: &BucketState, previous: &Option<BucketState>) -> bool;
}

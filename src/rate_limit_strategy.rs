use crate::bucket_state::BucketState;
use crate::window::TimeWindow;

pub trait RateLimitStrategy {
    fn is_rate_limited(&self, window: &TimeWindow, current: &BucketState, previous: &Option<BucketState>); 
}

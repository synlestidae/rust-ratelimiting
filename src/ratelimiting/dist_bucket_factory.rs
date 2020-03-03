use crate::ratelimiting::RateLimitStrategy;
use crate::periodic::UpdateTracker;
use crate::periodic::UpdateStrategy;
use crate::time::TimeWindow;
use crate::ratelimiting::DistBucketState;

pub trait DistBucketFactory: Clone {
    type R: RateLimitStrategy; 
    type T: UpdateTracker; 
    type S: UpdateStrategy; 

    fn make(&mut self, key: &str, window: &TimeWindow) -> DistBucketState<Self::R, Self::T, Self::S>;
}

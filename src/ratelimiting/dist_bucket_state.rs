use crate::periodic::UpdateTracker;
use crate::periodic::UpdateStrategy;
use crate::ratelimiting::RateLimitStrategy;
//use crate::store::GlobalStore;
use crate::bucket::BucketState;
use crate::periodic::UpdateState;

#[derive(Clone)]
pub struct DistBucketState<R: RateLimitStrategy, U: UpdateTracker, S: UpdateStrategy> {
    pub bucket_state: BucketState,
    pub update_tracker: U,
    pub update_strategy: S,
    pub rate_limit_strategy: R,
    //pub store: G,
    pub state: Option<UpdateState>,
}

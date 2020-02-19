use crate::bucket::BucketState;
use crate::periodic::GlobalIncrement;
use crate::periodic::UpdateState;
use crate::periodic::UpdateValue;

pub trait UpdateTracker {
    fn from(bucket: &BucketState) -> Self;

    fn build_update(&mut self, bucket_state: &mut BucketState) -> UpdateState;
}

use crate::bucket::BucketState;
use crate::periodic::UpdateState;

pub trait UpdateTracker {
    fn from(bucket: &BucketState) -> Self;

    fn build_update(&mut self, bucket_state: &mut BucketState) -> UpdateState;
}

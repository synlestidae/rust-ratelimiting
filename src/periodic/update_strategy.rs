use crate::bucket::BucketState;

pub trait UpdateStrategy {
    fn from(bucket: &BucketState) -> Self;

    fn needs_update(&mut self, bucket_state: &BucketState) -> bool;
}

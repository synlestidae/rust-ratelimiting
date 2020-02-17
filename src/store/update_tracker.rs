pub trait UpdateTracker {
    fn from(bucket: &BucketState) -> Self;

    fn needs_increment(&self) -> bool;

    fn build_increment(&mut self) -> GlobalIncrement;
}

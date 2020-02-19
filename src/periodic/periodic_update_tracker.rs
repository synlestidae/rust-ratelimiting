use crate::bucket::BucketState;
use crate::periodic::UpdateState;
use crate::periodic::UpdateValue;
use crate::periodic::GlobalIncrement;
use crate::periodic::UpdateTracker;
use rand::Rng;

pub struct PeriodicUpdateTracker {
    state: Option<UpdateState>,
    key: String,
}

impl UpdateTracker for PeriodicUpdateTracker {
    fn from(bucket: &BucketState) -> Self {
        Self {
            state: None as Option<UpdateState>,
            key: bucket.key.to_owned(),
        }
    }

    fn build_update(&mut self, bucket_state: &mut BucketState) -> UpdateState {
        let increment = bucket_state.clear_local_count();

        let new_state = UpdateState::new(&bucket_state.key, increment);

        self.state = Some(new_state.clone());

        new_state
    }
}

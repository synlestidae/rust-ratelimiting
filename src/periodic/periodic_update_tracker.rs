use crate::bucket::BucketState;
use crate::periodic::UpdateState;
use crate::periodic::UpdateValue;
use crate::periodic::GlobalIncrement;
use crate::periodic::UpdateTracker;
use rand::Rng;

pub struct PeriodicUpdateTracker {
    state: Option<UpdateState>,
    key: String,
    global_update_count: u32,
    desired_total_updates: u32,
}

impl UpdateTracker for PeriodicUpdateTracker {
    fn from(bucket: &BucketState) -> Self {
        Self {
            state: None as Option<UpdateState>,
            key: bucket.key.to_owned(),
            global_update_count: 1,
            desired_total_updates: DEFAULT_DESIRED_TOTAL_UPDATES + 1
        }
    }

    fn needs_update(&self, bucket_state: &BucketState) -> bool {
        if self.state.is_some() {
            false
        } else {
            bucket_state.get_count() / self.global_update_count >= self.desired_total_updates
        }
    }

    fn build_update(&mut self, bucket_state: &mut BucketState) -> UpdateState {
        let increment = bucket_state.clear_local_count();

        self.global_update_count += 1;

        let new_state = UpdateState::new(&bucket_state.key, increment);

        self.state = Some(new_state.clone());

        new_state
    }

    fn poll_update(&mut self) -> Option<UpdateValue> {
        if let Some(ref mut state) = &mut self.state {
            if state.is_failed() {
                // do nothing
                self.state = None;
            }
            else if state.is_done() {
                let global_value = state.global_value();

                self.state = None;

                return Some(UpdateValue::new(&self.key, global_value));
            }
        }

        None
    }
}

const DEFAULT_DESIRED_TOTAL_UPDATES: u32 = 10;

use crate::bucket_state::BucketState;
use crate::update_state::UpdateState;
use std::sync::mpsc::channel;

pub struct UpdateTracker {
    state: Option<UpdateState>,
    global_value: u32,
    global_update_count: u32,
    global_update_threshold: u32
}

impl UpdateTracker {
    pub fn new(global_value: u32, global_update_threshold: u32) -> Self {
        Self {
            state: None,
            global_value: 0,
            global_update_count: 1,
            global_update_threshold

        }
    }

    pub fn refresh(&mut self) -> Option<u32> {
        if let Some(ref mut state) = &mut self.state {
            if state.is_failed() {
                // do nothing
                self.state = None;
            }
            else if state.is_done() {
                let global_value = state.global_value();

                self.state = None;

                return Some(global_value);

            }
        }

        None
    }

    pub fn needs_update(&self, bucket_state: &BucketState) -> bool {
        if self.state.is_some() {
            false
        } else {
             bucket_state.get_count() / self.global_update_count >= self.global_update_threshold
        }
    }

    pub fn prep_update(&mut self, bucket_state: &mut BucketState) -> UpdateState {
        // prep the update package
        let increment = bucket_state.clear_local_count();

        self.global_update_count += 1;

        let new_state = UpdateState::new(&bucket_state.key, increment);

        self.state = Some(new_state.clone());

        new_state
    }
}

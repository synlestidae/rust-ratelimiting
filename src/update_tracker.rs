use crate::bucket_state::BucketState;
use crate::update_anchor::UpdateAnchor;
use crate::update_line::UpdateLine;
use crate::update_state::UpdateState;
use crate::update_package::UpdatePackage;
use std::sync::mpsc::channel;

pub struct UpdateTracker {
    state: UpdateState,
    global_value: u32,
    global_update_count: u32,
    global_update_threshold: u32
}

impl UpdateTracker {
    pub fn new(global_value: u32, global_update_threshold: u32) -> Self {
        Self {
            state: UpdateState::Local,
            global_value,
            global_update_count: 0,
            global_update_threshold

        }
    }
    pub fn needs_update(&self, bucket_state: &BucketState) -> bool {
        if self.state.is_busy() {
            false
        } else {
            bucket_state.count / self.global_update_threshold >= self.global_update_count
        }
    }

    pub fn prep_update(&mut self, bucket_state: &BucketState) -> UpdateLine {
        // prep the update package
        let update_package = UpdatePackage::new(&bucket_state.key, &bucket_state.window, bucket_state.count);

        // create the channel between UpdateAnchor and UpdateLine
        let (send, recv) = channel();
        let update_anchor = UpdateAnchor::new(recv, update_package.clone());

        // change the state
        self.state = UpdateState::PendingGlobalIncrement { 
            last_global_value: self.global_value, 
            update_anchor
        };

        UpdateLine::new(send, update_package)
    }

    pub fn commit_global_increment(&mut self, bucket_state: &mut BucketState) {
        unimplemented!()
    }

    pub fn commit_global_read(self, bucket_state: &mut BucketState, global_value: u32) {
        unimplemented!()
    }
}

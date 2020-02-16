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
            global_value: 0,
            global_update_count: 1,
            global_update_threshold

        }
    }

    pub fn refresh(&mut self) -> Option<(u32, u32)> {
        let (is_done, last_global_value, current_global_value) = {
            let (update_anchor, last_global_value) = match self.state {
                UpdateState::PendingGlobalIncrement { last_global_value, ref mut update_anchor } => (update_anchor, last_global_value),
                UpdateState::PendingGlobalRead { last_global_value, ref mut update_anchor } => (update_anchor, last_global_value),
                _ => return None
            };

            update_anchor.check(); 

            (update_anchor.is_done(), last_global_value, update_anchor.read_global_value)
        };


        if is_done {
            self.state = UpdateState::Local;
        }

        match (is_done, last_global_value, current_global_value) {
            (true, last, Some(current)) => {
                self.global_value = current;

                return Some((last, current));
            },
            _ => return None
        };

        None
    }

    pub fn needs_update(&self, bucket_state: &BucketState) -> bool {
        if self.state.is_busy() {
            false
        } else {
             bucket_state.get_count() / self.global_update_count >= self.global_update_threshold
        }
    }

    pub fn prep_update(&mut self, bucket_state: &mut BucketState) -> UpdateLine {
        // prep the update package
        let increment = bucket_state.clear_local_count();

        let update_package = UpdatePackage::new(&bucket_state.key, &bucket_state.window, increment);

        // create the channel between UpdateAnchor and UpdateLine
        let (send, recv) = channel();
        let update_anchor = UpdateAnchor::new(recv, update_package.clone());

        // change the state
        self.state = UpdateState::PendingGlobalIncrement { 
            last_global_value: self.global_value, 
            update_anchor
        };

        self.global_update_count += 1;

        UpdateLine::new(send, update_package)
    }

    pub fn commit_global_increment(&mut self, bucket_state: &mut BucketState) {
        unimplemented!()
    }

    pub fn commit_global_read(self, bucket_state: &mut BucketState, global_value: u32) {
        unimplemented!()
    }
}

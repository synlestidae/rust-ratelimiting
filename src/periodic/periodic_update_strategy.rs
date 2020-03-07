use crate::bucket::BucketState;
use crate::periodic::UpdateStrategy;

pub struct PeriodicUpdateStrategy {
    global_update_count: u32,
    desired_total_updates: u32,
}

impl UpdateStrategy for PeriodicUpdateStrategy {
    fn from(_: &BucketState) -> Self {
        Self {
            global_update_count: 1,
            desired_total_updates: DEFAULT_DESIRED_TOTAL_UPDATES + 1
        }
    }

    fn needs_update(&mut self, bucket_state: &BucketState) -> bool {
        let needs_update = bucket_state.get_count() / self.global_update_count >= self.desired_total_updates;
        if needs_update {
            self.global_update_count += 1;
        }

        needs_update
    }
}

const DEFAULT_DESIRED_TOTAL_UPDATES: u32 = 10;

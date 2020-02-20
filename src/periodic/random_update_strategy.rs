use crate::periodic::UpdateStrategy;
use crate::bucket::BucketState;
use rand::prelude::*;

pub struct RandomUpdateStrategy {
    rng: ThreadRng,
    next_update_at: u32,
    rate_limit_triggered: bool,
    is_new: bool
}

impl UpdateStrategy for RandomUpdateStrategy {
    fn from(bucket: &BucketState) -> Self {
        let mut rng = thread_rng();
        println!("From {:?}", bucket);
        let next_update_at = next_update_at(&mut rng, bucket.get_count(), bucket.limit);

        Self {
            rng,
            next_update_at,
            rate_limit_triggered: false,
            is_new: true
        }
    }

    fn needs_update(&mut self, bucket_state: &BucketState) -> bool {
        if self.is_new {
            self.is_new = false;
            return true;
        }

        if bucket_state.get_count() >= bucket_state.limit {
            self.rate_limit_triggered = true;
            return true;
        }

        let needs_update = bucket_state.get_count() >= self.next_update_at;

        if needs_update {
            self.next_update_at = next_update_at(&mut self.rng, bucket_state.get_count(), bucket_state.limit);
            println!("Next update at {} for {} of {}", self.next_update_at, bucket_state.get_count(), bucket_state.limit);
        }

        needs_update
    }
}

fn next_update_at(tr: &mut ThreadRng, start: u32, limit: u32) -> u32 {
    println!("gen_range {}, {}", start, start + (limit / DEFAULT_DESIRED_TOTAL_UPDATES));
    tr.gen_range(start, start + (limit / DEFAULT_DESIRED_TOTAL_UPDATES))
}

const DEFAULT_DESIRED_TOTAL_UPDATES: u32 = 20;

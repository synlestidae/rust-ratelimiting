use std::time::{Instant, Duration};
use std::cmp::{Eq, PartialEq};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeWindow {
    start: Instant,
    length: Duration
}

impl TimeWindow {
    pub fn new(start: Instant, length: Duration) -> Self {
        Self {
            start,
            length
        }
    }

    pub fn contains(&self, instant: Instant) -> bool {
        return self.start <= instant && instant < self.start + self.length;
    }

    pub fn is_next(&self, other: &TimeWindow) -> bool {
        unimplemented!()
    }

    pub fn is_after(&self, other: &TimeWindow) -> bool {
        unimplemented!()
    }
}

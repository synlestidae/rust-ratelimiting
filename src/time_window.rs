use std::cmp::{Eq, PartialEq};
use chrono::Duration;
use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeWindow {
    start: DateTime<Utc>,
    length: Duration
}

impl TimeWindow {
    pub fn new(start: DateTime<Utc>, length: Duration) -> Self {
        Self {
            start,
            length
        }
    }

    pub fn contains(&self, instant: DateTime<Utc>) -> bool {
        return self.start <= instant && instant < self.start + self.length;
    }

    pub fn is_next(&self, other: &TimeWindow) -> bool {
        self.start >= other.start + other.length
    }

    pub fn is_after_next(&self, other: &TimeWindow) -> bool {
        self.start + self.length >= other.start
    }

    pub fn slide_ratio(&self, other: &DateTime<Utc>) -> f64 {
        let instance_milliseconds = f64::from(other.signed_duration_since(self.start.clone()).num_milliseconds() as i32);
        let window_milliseconds = f64::from(self.length.num_milliseconds() as i32);

        instance_milliseconds / window_milliseconds
    }
}

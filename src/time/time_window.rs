use std::cmp::{Eq, PartialEq};
use chrono::Duration;
use chrono::DateTime;
use chrono::offset::Utc;
use chrono::NaiveDateTime;
use chrono::Timelike;
use chrono::Datelike;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeWindow {
    start: DateTime<Utc>,
    duration: Duration
}

impl TimeWindow {
    pub fn new(start: DateTime<Utc>, duration: Duration) -> Self {
        Self {
            start,
            duration
        }
    }

    pub fn from(instance: DateTime<Utc>, duration: Duration) -> Self {
        // convert to millis
        let instance_millis = instance.timestamp_millis();
        let duration_millis = duration.num_milliseconds();
        let window_millis = instance_millis - (instance_millis % duration_millis);
        let start = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(window_millis / 1000, 0), Utc);

        Self {
            start,
            duration
        }
    }

    pub fn contains(&self, instant: DateTime<Utc>) -> bool {
        return self.start <= instant && instant < self.start + self.duration;
    }

    pub fn is_next(&self, other: &TimeWindow) -> bool {
        self.start >= other.start + other.duration
    }

    pub fn is_after_next(&self, other: &TimeWindow) -> bool {
        self.start + self.duration >= other.start
    }

    pub fn slide_ratio(&self, other: &DateTime<Utc>) -> f64 {
        let instance_milliseconds = f64::from(other.signed_duration_since(self.start.clone()).num_milliseconds() as i32);
        let window_milliseconds = f64::from(self.duration.num_milliseconds() as i32);

        instance_milliseconds / window_milliseconds
    }

    pub fn key(&self) -> String {
        let date = self.start.date();
        let time = self.start.time();

        format!("{}-{}-{}-{}-{}-{}", date.year(), date.month(), date.day(), time.hour(), time.minute(), time.second())
    }
}

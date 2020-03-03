use crate::time::TimeWindow;
use chrono::Duration;
use chrono::{NaiveDate, DateTime, Utc};


#[test]
fn creates_instant() {
    TimeWindow::new(Utc::now(), Duration::minutes(1));
}

#[test]
fn has_next_window() {
    let time_window = TimeWindow::new(Utc::now(), Duration::minutes(1));
    let next_time_window = TimeWindow::new(Utc::now() + Duration::minutes(1), Duration::minutes(1));

    assert!(next_time_window.is_next(&time_window));
}

#[test]
fn has_after_next_window() {
    let time_window = TimeWindow::new(Utc::now(), Duration::minutes(1));
    let next_time_window = TimeWindow::new(Utc::now() + Duration::minutes(1), Duration::minutes(1));

    assert!(next_time_window.is_after_next(&time_window));
}

#[test] 
fn window_tests() {
    let now = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 0), Utc);
    let just_a_bit_later = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 11, 0), Utc);
    let quite_a_bit_later = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 16, 0), Utc);

    let window_duration = Duration::minutes(5);

    assert_eq!(TimeWindow::from(now, window_duration), TimeWindow::from(just_a_bit_later, window_duration));
    assert_ne!(TimeWindow::from(now, window_duration), TimeWindow::from(quite_a_bit_later, window_duration));
}

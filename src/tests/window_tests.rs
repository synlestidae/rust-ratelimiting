use crate::time_window::TimeWindow;
use chrono::Duration;
use chrono::offset::Utc;

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

use crate::bucket_state::BucketState;
use crate::time_window::TimeWindow;

use chrono::Duration;
use chrono::offset::Utc;
use crate::sliding_window_rate_limit_strategy::SlidingWindowRateLimitStrategy;

#[test]
fn bucket_increments_by_1() {
    let start = Utc::now();
    let window_duration = Duration::minutes(1);
    let window = TimeWindow::new(start, window_duration);
    let mut bucket_state = BucketState::new("test", &window, 5);

    assert_eq!(bucket_state.increment(1, &window), 1);
}

#[test]
fn bucket_increments_by_2() {
    let start = Utc::now();
    let window_duration = Duration::minutes(1);
    let window = TimeWindow::new(start, window_duration);
    let mut bucket_state = BucketState::new("test", &window, 5);

    assert_eq!(bucket_state.increment(2, &window), 2);
}

#[test]
fn bucket_increments_by_2_when_increment_twice() {
    let start = Utc::now();
    let window_duration = Duration::minutes(1);
    let window = TimeWindow::new(start, window_duration);
    let mut bucket_state = BucketState::new("test", &window, 5);

    bucket_state.increment(1, &window);

    assert_eq!(bucket_state.increment(1, &window), 2);
}

#[test]
fn bucket_incremented_resets_when_window_moves() {
    let start = Utc::now();
    let window_duration = Duration::minutes(1);
    let mut window = TimeWindow::new(start, window_duration);
    let mut bucket_state = BucketState::new("test", &window, 5);

    bucket_state.increment(1, &window);
    bucket_state.increment(4, &window);

    window = TimeWindow::new(start + window_duration * 2, window_duration);

    bucket_state.increment(10, &window);

    assert_eq!(bucket_state.get_count(), 10);
}

#[test]
fn bucket_is_rate_limited_by_sliding_window() {
    let start = Utc::now();
    let window_duration = Duration::minutes(1);
    let first_window = TimeWindow::new(start, window_duration);
    let second_window = TimeWindow::new(start + window_duration, window_duration);
    let mut bucket_state = BucketState::new("test", &first_window, 10);

    bucket_state.increment(10, &first_window);
    bucket_state.increment(5, &second_window);

    assert!(bucket_state.is_rate_limited(start + window_duration + Duration::seconds(5), &SlidingWindowRateLimitStrategy{}));
    assert!(!bucket_state.is_rate_limited(start + window_duration + Duration::seconds(31), &SlidingWindowRateLimitStrategy{}));
}

#[test]
fn bucket_not_incremented_when_sliding_window_passes() {
    let start = Utc::now();
    let window_duration = Duration::minutes(1);
    let first_window = TimeWindow::new(start, window_duration);
    let second_window = TimeWindow::new(start + window_duration, window_duration);
    let mut bucket_state = BucketState::new("test", &first_window, 10);

    bucket_state.increment(10, &first_window);
    bucket_state.increment(1, &second_window);
    bucket_state.increment(1, &second_window);

    assert!(!bucket_state.is_rate_limited(start + Duration::seconds(60 * 2), &SlidingWindowRateLimitStrategy{}));
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

use crate::time::TimeWindow;

pub trait Bucket {
    fn get_count(&self) -> u32;

    fn get_window(&self) -> &TimeWindow;

    fn next(&self, window: &TimeWindow) -> Self;

    fn increment(&mut self, delta: u32, window: &TimeWindow) -> u32;

    fn clear_local_count(&mut self) -> u32;

    fn set_global_count(&mut self, global_count: u32);
}

use crate::time_window::TimeWindow;

#[derive(Clone, Debug)]
pub struct UpdatePackage {
    pub key: String,
    pub time_window: TimeWindow,
    pub global_increment: u32,
}

impl UpdatePackage {
    pub fn new(key: &str, time_window: &TimeWindow, global_increment: u32) -> Self {
        Self {
            key: key.to_string(),
            time_window: time_window.clone(),
            global_increment
        }
    }
}

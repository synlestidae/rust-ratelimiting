#[derive(Debug)]
pub struct UpdateValue {
    pub key: String,
    pub global_value: u32
}

impl UpdateValue {
    pub fn new<S: Into<String>>(key: S, global_value: u32) -> Self {
        Self {
            key: key.into(),
            global_value
        }
    }
}

pub struct GlobalIncrement {
    pub key: String,
    pub increment: u32
}

impl GlobalIncrement {
    pub fn new<S: Into<String>>(key: S, increment: u32) -> Self {
        Self {
            key: key.into(),
            increment
        }
    }
}

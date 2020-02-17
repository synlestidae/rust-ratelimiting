use std::error::Error;

pub struct StoreError {
    pub error: Box<dyn Error>
}

impl<E: Error> From<E> for StoreError {
    fn from(e: E) -> Self {
        Self {
            error: Box::new(e)
        }
    }
}

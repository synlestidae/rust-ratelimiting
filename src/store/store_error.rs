use std::error::Error;

#[derive(Debug)]
pub struct StoreError {
    pub error: Box<dyn Error>
}

impl<E: Error + 'static> From<E> for StoreError {
    fn from(e: E) -> Self {
        Self {
            error: Box::new(e)
        }
    }
}

use std::err:Error;

pub struct StoreError {
    pub error: Box<Error>
}

impl<E: Error> From<E> for StoreError {
    fn from(e: E) -> Self {
        Self {
            error: Box::new(e)
        }
    }
}

impl StoreError {
    pub fn from<E: Error>(err: 
}



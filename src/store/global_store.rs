use crate::store::StoreError;

pub trait GlobalStore {
    fn set(&self, key: &str, val: u32) -> Result<(), StoreError>;

    fn get(&self, key: &str, val: u32) -> Result<u32, StoreError>;

    fn increment(&self, key: &str, val: u32) -> Result<(), StoreError>;
}

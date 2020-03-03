use redis::Client;
use redis::ConnectionInfo;
use crate::store::StoreError;
use crate::store::GlobalStore;

pub struct RedisStore {
    connection_info: ConnectionInfo
}

impl RedisStore {
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self {
            connection_info
        }
    }
}

impl GlobalStore for RedisStore {
    fn set(&self, key: &str, val: u32) -> Result<(), StoreError> {
        let mut client = Client::open(self.connection_info.clone())?;

        redis::cmd("SET")
            .arg(key)
            .arg(&val.to_string())
            .query::<u32>(&mut client)?;

        Ok(())
    }

    fn get(&self, key: &str) -> Result<u32, StoreError> {
        let mut client = Client::open(self.connection_info.clone())?;

        let result = redis::cmd("GET")
            .arg(key)
            .query::<u32>(&mut client)?;

        Ok(result)
    }

    fn increment(&self, key: &str, val: u32) -> Result<(), StoreError> {
        let mut client = Client::open(self.connection_info.clone())?;

        redis::cmd("INCRBY")
            .arg(key)
            .arg(&val.to_string())
            .query(&mut client)?;

        Ok(())
    }
}

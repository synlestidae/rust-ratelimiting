use std::sync::Mutex;
use std::sync::Arc;
use std::ops::Drop;

#[derive(Clone, Debug)]
pub struct UpdateState {
    state: Arc<Mutex<InnerState>>
}

impl UpdateState {
    pub fn new(key: &str, global_increment: u32) -> Self {
        Self {
            state: Arc::new(Mutex::new(InnerState {
                global_increment,
                is_failed: false,
                is_done: false,
                global_value: 0,
                key: key.to_string()

            })
            )
        }
    }

    pub fn global_increment(&self) -> u32 {
        match self.state.lock() {
            Ok(state) => state.global_increment,
            _ => 0
        }
    }

    pub fn key(&self) -> Option<String> {
        match self.state.lock() {
            Ok(state) => Some(state.key.to_owned()),
            _ => None 
        }
    }

    pub fn is_done(&self) -> bool {
        match self.state.lock() {
            Ok(guard) => guard.is_done,
            Err(_) => true
        }
    }

    pub fn is_failed(&self) -> bool {
        match self.state.lock() {
            Ok(guard) => guard.is_failed,
            Err(_) => true
        }
    }

    pub fn global_value(&self) -> u32 {
        match self.state.lock() {
            Ok(guard) => guard.global_value,
            Err(_) => 0
        }
    }

    pub fn read_success(&self, read_value: u32) {
        match self.state.lock() {
            Ok(ref mut state) => { 
                state.global_value = read_value;
                state.is_done = true;
            },
            Err(_) => {}
        }
    }
}

impl Drop for UpdateState {
    fn drop(&mut self) {
        match self.state.lock() {
            Ok(ref mut state) => state.is_failed = !state.is_done,
            Err(_) => {}
        }
    }
}

#[derive(Clone, Debug)]
struct InnerState {
    global_increment: u32,
    key: String,
    is_failed: bool,
    is_done: bool,
    global_value: u32
}

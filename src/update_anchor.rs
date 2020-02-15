use std::sync::mpsc::Receiver;
use crate::update_message::UpdateMessage;
use crate::update_package::UpdatePackage;

pub struct UpdateAnchor { 
    pub is_failed: bool,
    pub is_increment_sent: bool,
    pub read_global_value: Option<u32>,
    recv: Receiver<UpdateMessage>
}

impl UpdateAnchor {
    pub fn new(recv: Receiver<UpdateMessage>, update_package: UpdatePackage) -> Self {
        Self {
            is_failed: false,
            is_increment_sent: false,
            read_global_value: None,
            recv
        }
    }

    pub fn check(&mut self) {
        for m in self.recv.try_iter() {
            match m {
                UpdateMessage::IncrementGlobalSucceeded { .. } => self.is_increment_sent = true,
                UpdateMessage::ReadGlobalSucceeded(new_value) => self.read_global_value = Some(new_value),
                UpdateMessage::Failed => self.is_failed = true
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_failed || self.is_increment_sent && !self.read_global_value.is_none()
    }
}

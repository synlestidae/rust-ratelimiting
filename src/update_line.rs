use std::sync::mpsc::Sender;
use crate::update_message::UpdateMessage;
use crate::update_package::UpdatePackage;

pub struct UpdateLine {
    pub update_package: UpdatePackage,
    is_done: bool,
    sender: Sender<UpdateMessage>
}

impl UpdateLine {
    pub fn new(sender: Sender<UpdateMessage>, update_package: UpdatePackage) -> Self {
        Self {
            is_done: false,
            update_package,
            sender
        }
    }

    pub fn read_global_succeeded(&self, value: u32) {
        self.sender.send(UpdateMessage::ReadGlobalSucceeded(value));
    }

    pub fn increment_global_succeeded(&self) {
        self.sender.send(UpdateMessage::IncrementGlobalSucceeded { update_package: self.update_package.clone() });
    }

    pub fn success(mut self) {
        self.is_done = true;
    }

    pub fn fail(mut self) {
        self.is_done = true;
        self.sender.send(UpdateMessage::Failed);
    }
}

impl Drop for UpdateLine {
    fn drop(&mut self) {
        if !self.is_done {
            self.sender.send(UpdateMessage::Failed);
        }

        self.is_done = true;
    }
}

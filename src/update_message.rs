use crate::update_package::UpdatePackage;

#[derive(Clone)]
pub enum UpdateMessage {
    IncrementGlobalSucceeded { update_package: UpdatePackage },
    ReadGlobalSucceeded(u32),
    Failed
}

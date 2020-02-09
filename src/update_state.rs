use crate::update_anchor::UpdateAnchor;

pub enum UpdateState {
    Local,
    LocalRead { last_global_value: u32 },
    PendingGlobalIncrement { last_global_value: u32, update_anchor: UpdateAnchor},
    PendingGlobalRead { last_global_value: u32, update_anchor: UpdateAnchor },
}

impl UpdateState {
    pub fn is_busy(&self) -> bool {
        match self {
            UpdateState::PendingGlobalIncrement { .. } => true,
            UpdateState::PendingGlobalRead { .. } => true,
            _ => false
        }
    }
}

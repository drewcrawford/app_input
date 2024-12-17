use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct PlatformCoalescedMouse {
}

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<crate::mouse::Shared>) -> Self {
        todo!()
    }
}
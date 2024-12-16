use std::ffi::c_void;
use std::sync::{Arc, Weak};
use crate::mouse::Shared;

#[derive(Debug)]
pub(crate) struct PlatformCoalescedMouse {
    imp: *mut c_void,
}

//swift side is Sendable
unsafe impl Send for PlatformCoalescedMouse {}
unsafe impl Sync for PlatformCoalescedMouse {}

#[no_mangle]
extern "C" fn raw_input_finish_mouse_event_context(ctx: *mut c_void) {
    todo!()
}

#[no_mangle]
extern "C" fn raw_input_mouse_move(ctx: *const c_void, abs_pos_x: f64, abs_pos_y: f64) {
    println!("abs_pos_x: {}, abs_pos_y: {}", abs_pos_x, abs_pos_y);
}



extern "C" {
    fn PlatformCoalescedMouseNew(ctx: *const c_void) -> *mut c_void;
    fn PlatformCoalescedMouseFree(imp: *mut c_void);
}

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<Shared>) -> Self {
        let weak = Arc::downgrade(shared);
        let weak_raw = Weak::into_raw(weak) as *const c_void;
        PlatformCoalescedMouse {
            imp: unsafe{PlatformCoalescedMouseNew(weak_raw)}
        }
    }
}

impl Drop for PlatformCoalescedMouse {
    fn drop(&mut self) {
        unsafe{PlatformCoalescedMouseFree(self.imp)}
    }
}
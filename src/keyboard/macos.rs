use std::ffi::c_void;
use std::sync::Arc;

pub struct PlatformCoalescedKeyboard {
    imp: *mut c_void,
}



unsafe extern "C" fn key_notify_func(ctx: *mut c_void) {
    println!("Hi from notify func")
}

extern "C" {
    fn PlatformCoalescedKeyboardNew(func: unsafe extern "C" fn (*mut c_void)) -> *mut c_void;
    fn PlatformCoalescedKeyboardFree(imp: *mut c_void);

    fn SwiftRawInputDebugWindowShow();
    fn SwiftRawInputDebugWindowHide();
}

pub fn debug_window_show() {
    unsafe { SwiftRawInputDebugWindowShow() }
}

pub fn debug_window_hide() {
    unsafe { SwiftRawInputDebugWindowHide() }
}

//Swift type implements Sendable
unsafe impl Send for PlatformCoalescedKeyboard {}
unsafe impl Sync for PlatformCoalescedKeyboard {}

impl PlatformCoalescedKeyboard {
    pub fn new(shared: Arc<crate::keyboard::Shared>) -> Self {
        PlatformCoalescedKeyboard {
            imp: unsafe { PlatformCoalescedKeyboardNew(key_notify_func) },
        }
    }
}

impl Drop for PlatformCoalescedKeyboard {
    fn drop(&mut self) {
        unsafe{PlatformCoalescedKeyboardFree(self.imp)}
    }
}
use std::cell::OnceCell;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock, Weak};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use crate::keyboard::sys::PlatformCoalescedKeyboard;
use crate::mouse::{MouseAbsoluteLocation, Shared};

struct MouseState {
    shareds: Vec<Weak<Shared>>
}
impl MouseState {
    fn new() -> Self {
        MouseState {
            shareds: Vec::new(),
        }
    }

    fn register_coalesced(&mut self, shared: &Arc<Shared>) {
        self.shareds.push(Arc::downgrade(shared));
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}

static MOUSE_STATE: OnceLock<Mutex<MouseState>> = OnceLock::new();
/**
Provide windows key events to raw_input.

# Returns
If we processed the message, returns LRESULT(0).  Otherwise returns non-zero.
*/
pub(crate) fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    LRESULT(1)

}

#[derive(Debug)]
pub struct PlatformCoalescedMouse {

}

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<Shared>) -> PlatformCoalescedMouse {
        MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap().register_coalesced(shared);
        PlatformCoalescedMouse {

        }
    }
}
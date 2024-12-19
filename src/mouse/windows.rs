use std::cell::OnceCell;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex, OnceLock, Weak};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, WM_MOUSEMOVE};
use crate::keyboard::sys::PlatformCoalescedKeyboard;
use crate::mouse::{MouseAbsoluteLocation, MouseWindowLocation, Shared};

fn get_x_lparam(lparam: LPARAM) -> i16 {
    ((lparam.0 as usize) & 0xFFFF) as u16 as i16
}

fn get_y_lparam(lparam: LPARAM) -> i16 {
    (((lparam.0 as usize) & 0xFFFF_0000) >> 16) as u16 as i16
}


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
    match msg {
        msg if msg == WM_MOUSEMOVE => {
            let x = get_x_lparam(l_param);
            let y = get_y_lparam(l_param);
            let mut point = MaybeUninit::uninit();
            let abs = unsafe{ClientToScreen(hwnd, point.as_mut_ptr())}.expect("failed to get client to screen");
            let point = unsafe{point.assume_init()};
            let abs_mouse = MouseAbsoluteLocation::new((point.x + x as i32) as f64, (point.y + y as i32) as f64);

            let mut rect = MaybeUninit::uninit();
            let client_area = unsafe{GetClientRect(hwnd,rect.as_mut_ptr())}.expect("failed to get client rect");

            let rect = unsafe{rect.assume_init()};
            let rel_mouse = MouseWindowLocation::new(x as f64, y as f64, rect.right as f64, rect.bottom as f64 );

            //take lock
            MOUSE_STATE.get_or_init(Mutex::default).lock().expect("can't lock").shareds.retain(|shared |{
               if let Some(shared) = shared.upgrade() {
                   shared.set_absolute_location(abs_mouse);
                   shared.set_window_location(rel_mouse);
                   true
               }
                else {
                    false
                }
            });
            LRESULT(0)
        }
        _ => LRESULT(1)
    }

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
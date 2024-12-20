#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

#[cfg(target_os = "windows")]
pub(crate) mod windows;

#[cfg(target_os = "linux")]
pub(crate) mod linux;

use std::ffi::c_void;
#[cfg(target_os = "macos")]
pub(crate) use macos as sys;

#[cfg(target_arch = "wasm32")]
pub(crate) use wasm as sys;

#[cfg(target_os = "windows")]
pub(crate) use windows as sys;

#[cfg(target_os="linux")]
pub(crate) use linux as sys;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use atomic_float::AtomicF64;
use crate::Window;

/**
Mouse's location in the window, in points.

Origin at the upper-left.
*/
#[derive(Debug,Clone,Copy)]
struct MouseWindowLocation {
    pos_x: f64,
    pos_y: f64,
    window_width: f64,
    window_height: f64,
    window: Option<Window>,
}

impl MouseWindowLocation {
    fn new(pos_x: f64, pos_y: f64, window_width: f64, window_height: f64, window: Option<Window>) -> Self {
        MouseWindowLocation{pos_x, pos_y, window_width, window_height, window}
    }
}







#[derive(Debug)]
struct Shared {
    window: std::sync::Mutex<Option<MouseWindowLocation>>,

    buttons: [AtomicBool; 255],
    scroll_delta_x: AtomicF64,
    scroll_delta_y: AtomicF64,
    last_window: AtomicPtr<c_void>,
}
impl Shared {
    fn new() -> Self {
        Shared{
            window: std::sync::Mutex::new(None),
            buttons: [const {AtomicBool::new(false)}; 255],
            scroll_delta_x: AtomicF64::new(0.0),
            scroll_delta_y: AtomicF64::new(0.0),
            last_window: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    fn set_window_location(&self, location: MouseWindowLocation) {
        logwise::debuginternal_sync!("Set mouse window location {location}",location=logwise::privacy::LogIt(&location));
        *self.window.lock().unwrap() = Some(location);
        self.last_window.store(location.window.map(|e| e.0.as_ptr()).unwrap_or(std::ptr::null_mut()), Ordering::Relaxed)
    }
    fn set_key_state(&self, key: u8, down: bool, window: *mut c_void) {
        logwise::debuginternal_sync!("Set mouse key {key} state {down}",key=key,down=down);
        self.buttons[key as usize].store(down, std::sync::atomic::Ordering::Relaxed);
        self.last_window.store(window, std::sync::atomic::Ordering::Relaxed);
    }

    fn add_scroll_delta(&self, delta_x: f64, delta_y: f64, window: *mut c_void) {
        logwise::debuginternal_sync!("Add mouse scroll delta {delta_x},{delta_y}",delta_x=delta_x,delta_y=delta_y);
        self.scroll_delta_x.fetch_add(delta_x, std::sync::atomic::Ordering::Relaxed);
        self.scroll_delta_y.fetch_add(delta_y, std::sync::atomic::Ordering::Relaxed);
        self.last_window.store(window, std::sync::atomic::Ordering::Relaxed);

    }
}



#[derive(Debug)]
pub struct Mouse {
    shared: Arc<Shared>,
    sys: sys::PlatformCoalescedMouse,
}

impl Mouse {
    /**
    Returns an object that coalesces input from all mice on the system.
*/
    pub fn coalesced() -> Self {
        let shared = Arc::new(Shared::new());
        let coalesced = sys::PlatformCoalescedMouse::new(&shared);
        Mouse{shared, sys: coalesced}
    }

    /**
    Returns the [MouseWindowLocation]

    # Platform specifics

    * macOS and wasm require no special considerations.
    * On windows, you must call [crate::window_proc] from your window.
    * * On Linux,you must call from appropriate wayland events:
        * [crate::mouse::linux::motion_event],
        * [crate::mouse::linux::button_event]
        * [crate::mouse::linux::xdg_toplevel_configure_event]
*/
    pub fn window_pos(&self) -> Option<MouseWindowLocation> {
        todo!()
    }
}

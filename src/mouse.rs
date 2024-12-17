#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

#[cfg(target_os = "windows")]
pub(crate) mod windows;


#[cfg(target_os = "macos")]
pub(crate) use macos as sys;

#[cfg(target_arch = "wasm32")]
pub(crate) use wasm as sys;

#[cfg(target_os = "windows")]
pub(crate) use windows as sys;

use std::iter::Scan;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};
use atomic_float::AtomicF64;

/**
Mouse's location in the window, in points.

Origin at the upper-left.
*/
#[derive(Debug)]
struct MouseWindowLocation {
    pos_x: f64,
    pos_y: f64,
    window_width: f64,
    window_height: f64,
}

impl MouseWindowLocation {
    fn new(pos_x: f64, pos_y: f64, window_width: f64, window_height: f64) -> Self {
        MouseWindowLocation{pos_x, pos_y, window_width, window_height}
    }
}



/**
Mouse's location.

Origin at the upper left.
*/
#[derive(Debug)]
struct MouseAbsoluteLocation {
    abs_pos_x: f64,
    abs_pos_y: f64,
}

impl MouseAbsoluteLocation {
    fn new(abs_pos_x: f64, abs_pos_y: f64) -> Self {
        MouseAbsoluteLocation{abs_pos_x, abs_pos_y}
    }
}



#[derive(Debug)]
struct Shared {
    abs: std::sync::Mutex<Option<MouseAbsoluteLocation>>,
    window: std::sync::Mutex<Option<MouseWindowLocation>>,

    buttons: [AtomicBool; 255],
    scroll_delta_x: AtomicF64,
    scroll_delta_y: AtomicF64,
}
impl Shared {
    fn new() -> Self {
        Shared{
            abs: std::sync::Mutex::new(None),
            window: std::sync::Mutex::new(None),
            buttons: [const {AtomicBool::new(false)}; 255],
            scroll_delta_x: AtomicF64::new(0.0),
            scroll_delta_y: AtomicF64::new(0.0),
        }
    }
    fn set_absolute_location(&self, location: MouseAbsoluteLocation) {
        logwise::debuginternal_sync!("Set mouse location {location}",location=logwise::privacy::LogIt(&location));
        *self.abs.lock().unwrap() = Some(location);
    }
    fn set_window_location(&self, location: MouseWindowLocation) {
        logwise::debuginternal_sync!("Set mouse window location {location}",location=logwise::privacy::LogIt(&location));
        *self.window.lock().unwrap() = Some(location);
    }
    fn set_key_state(&self, key: u8, down: bool) {
        logwise::debuginternal_sync!("Set mouse key {key} state {down}",key=key,down=down);
        self.buttons[key as usize].store(down, std::sync::atomic::Ordering::Relaxed);
    }

    fn add_scroll_delta(&self, delta_x: f64, delta_y: f64) {
        logwise::debuginternal_sync!("Add mouse scroll delta {delta_x},{delta_y}",delta_x=delta_x,delta_y=delta_y);
        self.scroll_delta_x.fetch_add(delta_x, std::sync::atomic::Ordering::Relaxed);
        self.scroll_delta_y.fetch_add(delta_y, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Drop for Shared {
    fn drop(&mut self) {
        panic!("why is this dropped?");
    }
}

#[derive(Debug)]
pub struct Mouse {
    shared: Arc<Shared>,
    sys: sys::PlatformCoalescedMouse,
}

impl Mouse {
    pub fn coalesced() -> Self {
        let shared = Arc::new(Shared::new());
        let coalesced = sys::PlatformCoalescedMouse::new(&shared);
        Mouse{shared, sys: coalesced}
    }
}

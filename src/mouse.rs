#[cfg(target_os = "macos")]
pub(crate) mod macos;

#[cfg(target_os = "macos")]
pub(crate) use macos as sys;

use std::sync::Arc;

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
}
impl Shared {
    fn new() -> Self {
        Shared{
            abs: std::sync::Mutex::new(None),
            window: std::sync::Mutex::new(None),
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

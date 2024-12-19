use std::sync::{Arc, Mutex, OnceLock,Weak};
use crate::mouse::{MouseWindowLocation, Shared};

#[derive(Debug)]
pub(super) struct PlatformCoalescedMouse {

}

struct MouseState {
    shareds: Vec<Weak<Shared>>,
    recent_x_pos: Option<f64>,
    recent_y_pos: Option<f64>,
    recent_window_width: Option<i32>,
    recent_window_height: Option<i32>,
}
impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            shareds: Vec::new(),
            recent_x_pos: None,
            recent_y_pos: None,
            recent_window_width: None,
            recent_window_height: None,
        }
    }
}

impl MouseState {
    fn apply_all<F: Fn(&Shared) -> ()>(&mut self, f: F) {
        self.shareds.retain(|shared| {
            if let Some(shared) = shared.upgrade() {
                f(&shared);
                true
            } else {
                false
            }
        })
    }
    fn send_events_if_needed(&mut self) {
        if let (Some(recent_window_width), Some(recent_window_height), Some(recent_x_pos), Some(recent_y_pos)) =
            (self.recent_window_width, self.recent_window_height, self.recent_x_pos, self.recent_y_pos){
            let pos = MouseWindowLocation::new(recent_x_pos, recent_y_pos, recent_window_width as f64, recent_window_height as f64);
            self.apply_all(|shared| {
                shared.set_window_location(pos);
            })
        }
    }
}

pub fn motion_event(time: u32, surface_x: f64, surface_y: f64) {
    let mut lock = MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap();
    lock.recent_x_pos = Some(surface_x);
    lock.recent_y_pos = Some(surface_y);
    lock.send_events_if_needed();
}

pub fn xdg_toplevel_configure_event(width: i32, height: i32) {
    let mut lock = MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap();
    lock.recent_window_width = Some(width);
    lock.recent_window_height = Some(height);
    lock.send_events_if_needed();
}


static MOUSE_STATE: OnceLock<Mutex<MouseState>> = OnceLock::new();

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<Shared>) -> Self {
        MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap().shareds.push(Arc::downgrade(shared));
        PlatformCoalescedMouse {

        }
    }
}
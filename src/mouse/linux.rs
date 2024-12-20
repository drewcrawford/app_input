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

pub fn motion_event(_time: u32, surface_x: f64, surface_y: f64) {
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

pub fn button_event(_time: u32, button: u32, state: u32) {
    let down = if state == 0 {
        false
    }
    else {
        true
    };
    //see https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
    let btn_code = match button {
        0x110 => 0, //BTN_LEFT
        0x111 => 1, //BTN_RIGHT
        0x112 => 2, //BTN_MIDDLE
        0x113 => 3, //BTN_SIDE
        0x114 => 4,//BTN_EXTRa
        0x115 => 5, //BTN_FORWARD
        0x116 => 6, //BTN_BACK
        0x117 => 7, //BTN_TASK
        0x118 => 8,
        0x119 => 9,
        _ => {
            println!("Unknown button code {:?}", button);
            return;
        }
        
    };
    MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap().apply_all(|shared| {
        shared.set_key_state(btn_code, down);
    })
}

pub fn axis_event(_time: u32, axis: u32, value: f64) {
    if axis == 0 {
        //vertical
        MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap().apply_all(|shared |{
            shared.add_scroll_delta(0.0,value);
        })
    }
    else { //horizontal
        MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap().apply_all(|shared |{
            shared.add_scroll_delta(value,0.0);
        })
    }
    
}


static MOUSE_STATE: OnceLock<Mutex<MouseState>> = OnceLock::new();

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<Shared>) -> Self {
        MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap().shareds.push(Arc::downgrade(shared));
        PlatformCoalescedMouse {

        }
    }
}
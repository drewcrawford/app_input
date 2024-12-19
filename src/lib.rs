pub mod keyboard;
pub mod mouse;


pub use keyboard::sys::{debug_window_show, debug_window_hide};


#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HWND, LRESULT,WPARAM, LPARAM};
#[cfg(target_os = "windows")]
pub fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if mouse::windows::window_proc(hwnd, msg, w_param, l_param) == LRESULT(0) {
        LRESULT(0)
    }
    else if keyboard::windows::kbd_window_proc(hwnd, msg, w_param, l_param) == LRESULT(0) {
        LRESULT(0)
    }
    else {
        LRESULT(1)
    }
}
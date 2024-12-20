/*!

input_gui is a cross-platform library for receiving keyboard and mouse events.

# Design principles

* Use the best modern backend on each platform
* Zero magic, this library can be easily used without interference alongside any other native code.
    * When native code may interfere, instead this library will be a no-op by default.  You must
      call into it from your eventloop before our events are delivered.
* Mouse events:
    * Mouse position is determined by the compositor.  Platform-specific acceleration will be applied.
        * This is appropriate for GUI apps and topdown strategy games, or anytime you have a system-drawn cursor.
          It is appropriate for some fullscreen games. It is not appropriate for Counter Strike.
    * Coordinates are translated into a platform-independent upper-left coordinate system that works everywhere
    * Mouse events may require the window to be 'active' to be delivered, per platform conventions
* Keyboard events:
   * Report raw up/down events whenever possible
        * We map hardware keys rather than unicode characters
        * If you are trying to implement text input, you have much work to do, including but not limited to the shift key
   * Keycodes are translated into a platform-independent enum that works everywhere


# Supported platforms
| Platform | Backend                  |
|----------|--------------------------|
| Windows  | win32*                   |
| macOS    | AppKit                   |
| Linux    | Wayland*                 |
| wasm32   | KeyboardEvent \| MouseEvent  |
| Yours    | Send a PR!               |


* `*`: Needs platform-native event integration before events are delivered

*/
///Provides information about keyboard events.
pub mod keyboard;
///Provides information about mouse events.
pub mod mouse;

pub use keyboard::sys::{debug_window_show, debug_window_hide};

/**
Provides information about the window an event was delivered to.

# Platform specifics
* on macOS, this is the pointer of an NSWindow.  No memory management is performed, so dereferencing the window may be invalid.
* on wasm32, we attach to the global DOM window, and we choose an opaque value arbitrarily for this type.
*/
#[derive(Debug,Hash,Eq,PartialEq,Copy,Clone)]
pub struct Window(pub std::ptr::NonNull<std::ffi::c_void>);

#[cfg(target_os="linux")]
pub mod linux {
    pub use crate::keyboard::linux::wl_keyboard_event;
    pub use crate::mouse::linux::{motion_event, button_event, xdg_toplevel_configure_event};
}

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
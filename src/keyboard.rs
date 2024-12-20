use std::ffi::c_void;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicPtr};


/**
keys on the keyboard
*/
pub mod key;

#[cfg(target_os = "macos")]
pub(crate) mod macos;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

#[cfg(target_os = "windows")]
pub(crate) mod windows;

#[cfg(target_os = "linux")]
pub(crate) mod linux;

#[cfg(target_os = "macos")]
pub(crate) use macos as sys;

#[cfg(target_arch = "wasm32")]
pub(crate) use wasm as sys;

#[cfg(target_os = "windows")]
pub(crate) use windows as sys;

#[cfg(target_os = "linux")]
pub(crate) use linux as sys;




use crate::keyboard::key::KeyboardKey;
use crate::keyboard::sys::PlatformCoalescedKeyboard;
use crate::Window;

struct Shared {
    key_states: Vec<AtomicBool>,
    window_ptr: AtomicPtr<c_void>,
}

impl Shared {
    fn new() -> Self {
        let mut vec = Vec::with_capacity(key::KeyboardKey::all_keys().len());
        for _ in 0..key::KeyboardKey::all_keys().len() {
            vec.push(AtomicBool::new(false));
        }
        Shared {
            key_states: vec,
            window_ptr: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    fn set_key_state(&self, key: KeyboardKey, state: bool, window_ptr: *mut c_void) {
        logwise::debuginternal_sync!("Setting key {key} to {state}",key=logwise::privacy::LogIt(key), state=state);
        self.window_ptr.store(window_ptr, std::sync::atomic::Ordering::Relaxed);
        self.key_states[key as usize].store(state, std::sync::atomic::Ordering::Relaxed);
    }
}



pub struct Keyboard {
    shared: Arc<Shared>,
    platform_coalesced_keyboard: PlatformCoalescedKeyboard,
}

impl Keyboard {
    /**
    Create a keyboard representing all coalesced keyboards on the system.
    */
    pub fn coalesced() -> Self {
        let shared = Arc::new(Shared::new());
        let platform_coalesced_keyboard = PlatformCoalescedKeyboard::new(&shared);
        Self {
            shared,
            platform_coalesced_keyboard,
        }
    }

    /**
    Determines if the key provided is pressed.

    # Platform specifics

    * macOS and wasm require no special considerations.
    * On windows, you must call [crate::window_proc] from your window.
    * On Linux,you must call [crate::linux::wl_keyboard_event] from your Wayland dispatch queue.

    */
    pub fn is_pressed(&self, key: KeyboardKey) -> bool {
        todo!()
    }

}


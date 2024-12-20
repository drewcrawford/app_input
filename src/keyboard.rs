use std::ffi::c_void;
use std::hash::Hash;
use std::marker::{PhantomData, PhantomPinned};
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

#[derive(Debug)]
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



#[derive(Debug)]
pub struct Keyboard {
    shared: Arc<Shared>,
    _platform_coalesced_keyboard: PlatformCoalescedKeyboard,
}

impl Keyboard {
    /**
    Create a keyboard representing all coalesced keyboards on the system.
    */
    pub fn coalesced() -> Self {
        let shared = Arc::new(Shared::new());
        let _platform_coalesced_keyboard = PlatformCoalescedKeyboard::new(&shared);
        Self {
            shared,
            _platform_coalesced_keyboard,
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
        self.shared.key_states[key as usize].load(std::sync::atomic::Ordering::Relaxed)
    }

}

//boilerplate

impl PartialEq for Keyboard {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.shared, &other.shared)
    }
}

impl Eq for Keyboard {}

impl Hash for Keyboard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.shared).hash(state);
    }
}

impl Default for Keyboard {
    /**
    The coalesced keyboard
    */
    fn default() -> Self {
        Self::coalesced()
    }
}

#[cfg(test)] mod test {
    use crate::keyboard::Keyboard;

    #[test] fn test_send_sync() {
        //I think basically the platform keyboard type operates as a kind of lifetime marker
        //(the main function is drop).  Accordingly it shouldn't be too bad to expect platforms to
        //implement send if necessary.
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        fn assert_unpin<T: Unpin>() {}

        assert_send::<Keyboard>();
        assert_sync::<Keyboard>();
        assert_unpin::<Keyboard>();
    }
}
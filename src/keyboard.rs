use std::sync::Arc;
use std::sync::atomic::AtomicBool;



pub mod key;

#[cfg(target_os = "macos")]
pub(crate) mod macos;

#[cfg(target_os = "macos")]
pub(crate) use macos as sys;
use crate::keyboard::key::KeyboardKey;
use crate::keyboard::sys::PlatformCoalescedKeyboard;

struct Shared {
    key_states: Vec<AtomicBool>,
}

impl Shared {
    fn new() -> Self {
        let mut vec = Vec::with_capacity(key::KeyboardKey::all_keys().len());
        for _ in 0..key::KeyboardKey::all_keys().len() {
            vec.push(AtomicBool::new(false));
        }
        Shared {
            key_states: vec,
        }
    }

    fn set_key_state(&self, key: KeyboardKey, state: bool) {
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

    pub fn is_pressed(&self, key: KeyboardKey) -> bool {
        todo!()
    }

}


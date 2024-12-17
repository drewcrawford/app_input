use std::sync::Arc;
use crate::keyboard::Shared;
use web_sys::KeyboardEvent;
use wasm_bindgen::prelude::*;


pub struct PlatformCoalescedKeyboard {

}

impl PlatformCoalescedKeyboard {
    pub fn new(shared: &Arc<Shared>) -> Self {
        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("no document on window");
        let keydown_callback = Closure::wrap(Box::new(|event: KeyboardEvent| {
            let key = event.key();
            let key_code = event.key_code();

            todo!("Process key {key:?} code {key_code:?}")

        }) as Box<dyn FnMut(KeyboardEvent)>);
        document
            .add_event_listener_with_callback(
                "keydown",
                keydown_callback.as_ref().unchecked_ref(),
            ).expect("Can't add event listener");
        keydown_callback.forget();

        Self {

        }

    }
}

pub fn debug_window_show() {
    //nothing?
}

pub fn debug_window_hide() {
    //also nothing?
}
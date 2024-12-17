use std::sync::Arc;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use crate::mouse::{MouseAbsoluteLocation, MouseWindowLocation};

fn js_button_to_rust(button: i16) -> u8 {
    match button {
        0 => 0,
        1 => 2,
        2 => 1,
        _ => button as u8,
    }
}

#[derive(Debug)]
pub(crate) struct PlatformCoalescedMouse {
    mouse_listener: JsValue,
    mousedown_listener: JsValue,
    mouseup_listener: JsValue,
}

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<crate::mouse::Shared>) -> Self {

        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("no document on window");

        let weak = Arc::downgrade(&shared);
        let weak_down = weak.clone();
        let weak_up = weak.clone();


        // Mouse move callback
        let mousemove_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Some(shared) = weak.upgrade() {
                shared.set_absolute_location(MouseAbsoluteLocation::new(event.client_x() as f64, event.client_y() as f64));
                let width = window
                    .inner_width()
                    .expect("failed to get width")
                    .as_f64()
                    .unwrap_or(0.0);

                let height = window
                    .inner_height()
                    .expect("failed to get height")
                    .as_f64()
                    .unwrap_or(0.0);


                shared.set_window_location(MouseWindowLocation::new(event.page_x() as f64, event.page_y() as f64, width, height));
            }
        }) as Box<dyn FnMut(MouseEvent)>);


        document
            .add_event_listener_with_callback(
                "mousemove",
                mousemove_callback.as_ref().unchecked_ref(),
            ).expect("Can't add event listener");

        let mousedown_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Some(shared) = weak_down.upgrade() {

                shared.set_key_state(js_button_to_rust(event.button()), true);
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        document.add_event_listener_with_callback(
            "mousedown",
            mousedown_callback.as_ref().unchecked_ref(),
        ).expect("Can't add event listener");

        let mouseup_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Some(shared) = weak_up.upgrade() {
                shared.set_key_state(js_button_to_rust(event.button()), false);
            }

        }) as Box<dyn FnMut(MouseEvent)>);
        document.add_event_listener_with_callback(
            "mouseup",
            mouseup_callback.as_ref().unchecked_ref(),
        ).expect("Can't add event listener");


        Self {
            mouse_listener: mousemove_callback.into_js_value(),
            mousedown_listener: mousedown_callback.into_js_value(),
            mouseup_listener: mouseup_callback.into_js_value(),
        }


    }
}
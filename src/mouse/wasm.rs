use std::sync::Arc;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;
use crate::mouse::{MouseAbsoluteLocation, MouseWindowLocation};

#[derive(Debug)]
pub(crate) struct PlatformCoalescedMouse {
    mouse_listener: JsValue,

}

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<crate::mouse::Shared>) -> Self {

        let window = web_sys::window().expect("no global window exists");
        let document = window.document().expect("no document on window");

        let weak = Arc::downgrade(&shared);


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

        Self {
            mouse_listener: mousemove_callback.into_js_value(),
        }


    }
}
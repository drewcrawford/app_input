use raw_input::keyboard::Keyboard;
use raw_input::{debug_window_show, debug_window_hide};
use raw_input::mouse::Mouse;

fn test_board() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let _k = Keyboard::coalesced();
    let _m = Mouse::coalesced();
    debug_window_show();
    debug_window_hide();

    //on wasm32 this thread completes
    #[cfg(target_arch="wasm32")] {
        std::mem::forget(_k);
        std::mem::forget(_m);
    }
}

fn main() {
    test_board();
}
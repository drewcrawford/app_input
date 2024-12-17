use raw_input::keyboard::Keyboard;
use raw_input::{debug_window_show, debug_window_hide};
use raw_input::keyboard::key::KeyboardKey;
use raw_input::mouse::Mouse;

fn test_board() {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let k = Keyboard::coalesced();
    let m = Mouse::coalesced();
    debug_window_show();
    std::thread::sleep(std::time::Duration::from_millis(10 * 1000));
    assert!(!k.is_pressed(KeyboardKey::A));
    println!("{:?}",m);
    debug_window_hide();
}

fn main() {
    test_board();
}
use raw_input::keyboard::Keyboard;
use raw_input::{debug_window_show, debug_window_hide};
use raw_input::keyboard::key::KeyboardKey;

fn test_board() {
    let k = Keyboard::coalesced();
    debug_window_show();
    std::thread::sleep(std::time::Duration::from_millis(10 * 1000));
    assert!(!k.is_pressed(KeyboardKey::A));
    debug_window_hide();
}

fn main() {
    test_board();
}
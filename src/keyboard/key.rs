// SPDX-License-Identifier: MPL-2.0
#[repr(usize)]
#[derive(Debug,Hash,Copy,Clone,PartialEq,Eq)]
#[non_exhaustive]
/**
A key on the keyboard.  Maps to an OS scancode.
*/
pub enum KeyboardKey {
    A,
    S,
    D,
    F,
    H,
    G,
    Z,
    X,
    C,
    V,
    B,
    Q,
    W,
    E,
    R,
    Y,
    T,
    Num1,
    Num2,
    Num3,
    Num4,
    Num6,
    Num5,
    Equal,
    Num9,
    Num7,
    Minus,
    Num8,
    Num0,
    RightBracket,
    O,
    U,
    LeftBracket,
    I,
    P,
    L,
    J,
    Quote,
    K,
    Semicolon,
    Backslash,
    Comma,
    Slash,
    N,
    M,
    Period,
    Grave,
    KeypadDecimal,
    KeypadMultiply,
    KeypadPlus,
    KeypadClear,
    KeypadDivide,
    KeypadEnter,
    KeypadMinus,
    KeypadEquals,
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Return,
    Tab,
    Space,
    Delete,
    Escape,
    Command,
    Shift,
    CapsLock,
    Option,
    Control,
    RightCommand,
    RightShift,
    RightOption,
    RightControl,
    Function,
    F17,
    VolumeUp,
    VolumeDown,
    Mute,
    F18,
    F19,
    F20,
    F5,
    F6,
    F7,
    F3,
    F8,
    F9,
    F11,
    F13,
    F16,
    F14,
    F10,
    ContextualMenu,
    F12,
    F15,
    Help,
    Home,
    PageUp,
    ForwardDelete,
    F4,
    End,
    F2,
    PageDown,
    F1,
    LeftArrow,
    RightArrow,
    DownArrow,
    UpArrow,
    ISOSection,
    JISYen,
    JISUnderscore,
    JISKeypadComma,
    JISEisu,
    JISKana,
    Pause,
    ScrollLock,
    PrintScreen,
    InternationalBackslash,
    F21,
    F22,
    F23,
    F24,
    Convert,
    NonConvert,
    PreviousTrack,
    NextTrack,
    LaunchApp2,
    Play,
    Stop,
    BrowserHome,
    NumLock,
    Insert,
    ContextMenu,
    Power,
    Eject,
    BrowserSearch,
    BrowserFavorites,
    BrowserRefresh,
    BrowserStop,
    BrowserForward,
    BrowserBack,
    LaunchApp1,
    LaunchMail,
    MediaSelect,
    Again,
    Props,
    Undo,
    Select,
    Copy,
    Open,
    Paste,
    Find,
    Cut,
    WakeUp,

}

impl KeyboardKey {
    /**
    Returns all keys supported by the library.
*/
    pub fn all_keys() -> Vec<KeyboardKey> {
        vec![
            KeyboardKey::A, KeyboardKey::S, KeyboardKey::D, KeyboardKey::F,
            KeyboardKey::H, KeyboardKey::G, KeyboardKey::Z, KeyboardKey::X,
            KeyboardKey::C, KeyboardKey::V, KeyboardKey::B, KeyboardKey::Q,
            KeyboardKey::W, KeyboardKey::E, KeyboardKey::R, KeyboardKey::Y,
            KeyboardKey::T, KeyboardKey::Num1, KeyboardKey::Num2,
            KeyboardKey::Num3, KeyboardKey::Num4, KeyboardKey::Num6,
            KeyboardKey::Num5, KeyboardKey::Equal, KeyboardKey::Num9,
            KeyboardKey::Num7, KeyboardKey::Minus, KeyboardKey::Num8,
            KeyboardKey::Num0, KeyboardKey::RightBracket, KeyboardKey::O,
            KeyboardKey::U, KeyboardKey::LeftBracket, KeyboardKey::I,
            KeyboardKey::P, KeyboardKey::L, KeyboardKey::J, KeyboardKey::Quote,
            KeyboardKey::K, KeyboardKey::Semicolon, KeyboardKey::Backslash,
            KeyboardKey::Comma, KeyboardKey::Slash, KeyboardKey::N,
            KeyboardKey::M, KeyboardKey::Period, KeyboardKey::Grave,
            KeyboardKey::KeypadDecimal, KeyboardKey::KeypadMultiply,
            KeyboardKey::KeypadPlus, KeyboardKey::KeypadClear,
            KeyboardKey::KeypadDivide, KeyboardKey::KeypadEnter,
            KeyboardKey::KeypadMinus, KeyboardKey::KeypadEquals,
            KeyboardKey::Keypad0, KeyboardKey::Keypad1, KeyboardKey::Keypad2,
            KeyboardKey::Keypad3, KeyboardKey::Keypad4, KeyboardKey::Keypad5,
            KeyboardKey::Keypad6, KeyboardKey::Keypad7, KeyboardKey::Keypad8,
            KeyboardKey::Keypad9, KeyboardKey::Return, KeyboardKey::Tab,
            KeyboardKey::Space, KeyboardKey::Delete, KeyboardKey::Escape,
            KeyboardKey::Command, KeyboardKey::Shift, KeyboardKey::CapsLock,
            KeyboardKey::Option, KeyboardKey::Control, KeyboardKey::RightCommand,
            KeyboardKey::RightShift, KeyboardKey::RightOption,
            KeyboardKey::RightControl, KeyboardKey::Function, KeyboardKey::F17,
            KeyboardKey::VolumeUp, KeyboardKey::VolumeDown, KeyboardKey::Mute,
            KeyboardKey::F18, KeyboardKey::F19, KeyboardKey::F20,
            KeyboardKey::F5, KeyboardKey::F6, KeyboardKey::F7, KeyboardKey::F3,
            KeyboardKey::F8, KeyboardKey::F9, KeyboardKey::F11, KeyboardKey::F13,
            KeyboardKey::F16, KeyboardKey::F14, KeyboardKey::F10,
            KeyboardKey::ContextualMenu, KeyboardKey::F12, KeyboardKey::F15,
            KeyboardKey::Help, KeyboardKey::Home, KeyboardKey::PageUp,
            KeyboardKey::ForwardDelete, KeyboardKey::F4, KeyboardKey::End,
            KeyboardKey::F2, KeyboardKey::PageDown, KeyboardKey::F1,
            KeyboardKey::LeftArrow, KeyboardKey::RightArrow,
            KeyboardKey::DownArrow, KeyboardKey::UpArrow, KeyboardKey::ISOSection,
            KeyboardKey::JISYen, KeyboardKey::JISUnderscore,
            KeyboardKey::JISKeypadComma, KeyboardKey::JISEisu, KeyboardKey::JISKana,
            KeyboardKey::Pause, KeyboardKey::ScrollLock, KeyboardKey::PrintScreen, KeyboardKey::InternationalBackslash,
            KeyboardKey::F21, KeyboardKey::F22, KeyboardKey::F23,
            KeyboardKey::F24,
            KeyboardKey::Convert, KeyboardKey::NonConvert,
            KeyboardKey::PreviousTrack, KeyboardKey::NextTrack,
            KeyboardKey::LaunchApp2, KeyboardKey::Play,
            KeyboardKey::Stop, KeyboardKey::BrowserHome,
            KeyboardKey::NumLock, KeyboardKey::Insert,
            KeyboardKey::ContextMenu, KeyboardKey::Power, KeyboardKey::Eject,
            KeyboardKey::BrowserSearch, KeyboardKey::BrowserFavorites,
            KeyboardKey::BrowserRefresh, KeyboardKey::BrowserStop,
            KeyboardKey::BrowserForward, KeyboardKey::BrowserBack,
            KeyboardKey::LaunchApp1, KeyboardKey::LaunchMail,
            KeyboardKey::MediaSelect, KeyboardKey::Again, KeyboardKey::Props,
            KeyboardKey::Undo, KeyboardKey::Select, KeyboardKey::Copy,
            KeyboardKey::Open, KeyboardKey::Paste, KeyboardKey::Find,
            KeyboardKey::Cut, KeyboardKey::WakeUp,


        ]

    }
}

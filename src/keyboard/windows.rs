use std::ffi::c_void;
use std::sync::Arc;
use windows::core::{w, PCWSTR};
use windows::Win32::Devices::HumanInterfaceDevice::{HID_USAGE_GENERIC_KEYBOARD, HID_USAGE_PAGE_GENERIC};
use windows::Win32::Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{COLOR_WINDOW, HBRUSH};
use windows::Win32::UI::Input::{RegisterRawInputDevices, RAWINPUTDEVICE, RIDEV_NOLEGACY};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, RegisterClassExW, RegisterClassW, ShowWindow, TranslateMessage, CW_USEDEFAULT, HMENU, IDC_ARROW, MSG, SW_SHOWNORMAL, WINDOW_EX_STYLE, WNDCLASSEXW, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW};
use crate::keyboard::Shared;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

pub struct PlatformCoalescedKeyboard {

}
impl PlatformCoalescedKeyboard {
    pub fn new(shared: &Arc<Shared>) -> Self {
        let device = RAWINPUTDEVICE {
            usUsagePage: HID_USAGE_PAGE_GENERIC,
            usUsage: HID_USAGE_GENERIC_KEYBOARD,
            //todo: the internet suggests that this may disable some windows behaviors
            dwFlags: RIDEV_NOLEGACY,
            // A handle to the target window. If NULL, raw input events follow the keyboard focus to ensure only the focused application window receives the events.
            hwndTarget: HWND(std::ptr::null_mut()),
        };
        unsafe{RegisterRawInputDevices(&[device], std::mem::size_of_val(&device) as u32)}
            .expect("failed to register raw input devices");
        PlatformCoalescedKeyboard {

        }

    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    println!("got msg hwnd {hwnd:?} msg {msg} w_param {w_param:?} l_param {l_param:?}");
    unsafe{DefWindowProcW(hwnd,msg,w_param, l_param)}
}

pub fn debug_window_show() {
    let instance = unsafe{GetModuleHandleW(PCWSTR::null())}.expect("Can't get module");
    let cursor = unsafe{LoadCursorW(HINSTANCE::default(), IDC_ARROW)}.expect("Can't load cursor");

    let class_name = w!("raw_input_debug_window");
    let window_class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: Default::default(),
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance.into(),
        hIcon: Default::default(),
        hCursor: cursor,
        hbrBackground: HBRUSH(COLOR_WINDOW.0 as usize as *mut c_void),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: class_name,
        hIconSm: Default::default(),
    };
    let r = unsafe{RegisterClassExW(&window_class)};
    assert_ne!(r, 0, "failed to register window class: {:?}",unsafe{GetLastError()});

    let window = unsafe{CreateWindowExW(WINDOW_EX_STYLE(0), //style
                                 class_name,
                                 w!("raw input debug window"),
                                 WS_OVERLAPPEDWINDOW,
                                 CW_USEDEFAULT, CW_USEDEFAULT, //position
                                 800, 600, //size
                                 HWND(std::ptr::null_mut()), //parent
                                 HMENU(std::ptr::null_mut()), //menu
                                 instance, //instance
                                        None,

    )}.expect("failed to create window");

    unsafe{_ = ShowWindow(window, SW_SHOWNORMAL)};

    // Message loop
    let mut msg = MSG::default();
    while (unsafe{GetMessageW(&mut msg, window, 0, 0).into()}) {
        let r: bool = unsafe{TranslateMessage(&msg)}.into();
        if r == true {
            panic!("failed to translate message");
        }
        unsafe{DispatchMessageW(&msg)};
    }

}
pub fn debug_window_hide() {
    todo!()
}
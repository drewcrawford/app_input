use std::sync::Arc;
use windows::Win32::Devices::HumanInterfaceDevice::{HID_USAGE_GENERIC_KEYBOARD, HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::{RegisterRawInputDevices, RAWINPUTDEVICE, RIDEV_NOLEGACY};
use crate::keyboard::sys::PlatformCoalescedKeyboard;
use crate::mouse::Shared;

#[derive(Debug)]
pub struct PlatformCoalescedMouse {

}

impl PlatformCoalescedMouse {
    pub fn new(shared: &Arc<Shared>) -> PlatformCoalescedMouse {
        let device = RAWINPUTDEVICE {
            usUsagePage: HID_USAGE_PAGE_GENERIC,
            usUsage: HID_USAGE_GENERIC_MOUSE,
            //todo: the internet suggests that this may disable some windows behaviors
            dwFlags: RIDEV_NOLEGACY,
            // A handle to the target window. If NULL, raw input events follow the keyboard focus to ensure only the focused application window receives the events.
            hwndTarget: HWND(std::ptr::null_mut()),
        };
        unsafe{RegisterRawInputDevices(&[device], std::mem::size_of_val(&device) as u32)}
            .expect("failed to register raw input devices");
        PlatformCoalescedMouse {

        }
    }
}
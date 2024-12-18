use std::cell::OnceCell;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock, Weak};
use windows::Win32::Devices::HumanInterfaceDevice::{HID_USAGE_GENERIC_KEYBOARD, HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::{GetRawInputData, RegisterRawInputDevices, HRAWINPUT, MOUSE_MOVE_ABSOLUTE, MOUSE_VIRTUAL_DESKTOP, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_NOLEGACY, RID_INPUT, RIM_TYPEMOUSE};
use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CXVIRTUALSCREEN, SM_CYSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, WM_INPUT};
use crate::keyboard::sys::PlatformCoalescedKeyboard;
use crate::mouse::{MouseAbsoluteLocation, Shared};

struct MouseState {
    shareds: Vec<Weak<Shared>>
}
impl MouseState {
    fn new() -> Self {
        MouseState {
            shareds: Vec::new(),
        }
    }

    fn register_coalesced(&mut self, shared: &Arc<Shared>) {
        self.shareds.push(Arc::downgrade(shared));
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}

static MOUSE_STATE: OnceLock<Mutex<MouseState>> = OnceLock::new();
/**
Provide windows key events to raw_input.

# Returns
If we processed the message, returns LRESULT(0).  Otherwise returns non-zero.
*/
pub(crate) fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        msg if msg == WM_INPUT => {
            let mut raw = RAWINPUT::default();
            let lparam_v = HRAWINPUT(l_param.0 as *mut c_void);
            let mut raw_size = std::mem::size_of_val(&raw) as u32;


            let data = unsafe {GetRawInputData(lparam_v, RID_INPUT, Some(&mut raw as *mut _ as  *mut c_void),
                                               &mut raw_size,
                                               std::mem::size_of::<RAWINPUTHEADER>() as u32)};
            assert!(data > 0);

            match raw.header.dwType {
                j if j == RIM_TYPEMOUSE.0 => {
                    let mouse = unsafe{raw.data.mouse};
                    let mouse_state = MOUSE_STATE.get_or_init(Mutex::default);

                    //https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-rawmouse
                    if mouse.usFlags.0 & MOUSE_MOVE_ABSOLUTE.0 != 0 && mouse.usFlags.0 & MOUSE_VIRTUAL_DESKTOP.0 != 0 {
                        let left = unsafe { GetSystemMetrics(SM_XVIRTUALSCREEN) };
                        let top = unsafe { GetSystemMetrics(SM_YVIRTUALSCREEN) };
                        let right = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
                        let bottom = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };
                        let abs_x = mouse.lLastX * right / 65535 + left;
                        let abs_y = mouse.lLastY * bottom / 65535 + top;
                        let abs_location = MouseAbsoluteLocation::new(abs_x as f64, abs_y as f64);
                        mouse_state.lock().unwrap().shareds.retain(|shared | {
                            if let Some(shared) = shared.upgrade() {
                                shared.set_absolute_location(abs_location);
                                true
                            } else {
                                false
                            }
                        });
                    } else if mouse.usFlags.0 & MOUSE_MOVE_ABSOLUTE.0 != 0 {
                        let right = unsafe{GetSystemMetrics(SM_CXSCREEN)};
                        let bottom = unsafe{GetSystemMetrics(SM_CYSCREEN)};
                        let abs_x = mouse.lLastX * right / 65535;
                        let abs_y = mouse.lLastY * bottom / 65535;
                        let abs_location = MouseAbsoluteLocation::new(abs_x as f64, abs_y as f64);
                        mouse_state.lock().unwrap().shareds.retain(|shared | {
                            if let Some(shared) = shared.upgrade() {
                                shared.set_absolute_location(abs_location);
                                true
                            } else {
                                false
                            }
                        });
                    }
                    else if mouse.lLastX != 0 || mouse.lLastY != 0 {
                        //relative position
                        let mut lock = mouse_state.lock().unwrap();
                        let mut absolute_position = None;
                        for item in &lock.shareds {
                            if let Some(shared) = item.upgrade() {
                                let mut abs_lock = shared.abs.lock().unwrap();

                                match absolute_position {
                                    None => {
                                        match abs_lock.as_mut() {
                                            Some(previous_abs) => {
                                                let new_abs = MouseAbsoluteLocation::new(previous_abs.abs_pos_x + mouse.lLastX as f64, previous_abs.abs_pos_y + mouse.lLastY as f64);
                                                absolute_position = Some(new_abs);
                                                *abs_lock = Some(new_abs);
                                            }
                                            None => {
                                                continue //try again
                                            }
                                        }
                                    }
                                    Some(new_abs) => {
                                        *abs_lock = Some(new_abs);
                                    }
                                }
                            }

                        }
                    }


                    LRESULT(0)

                }
                _ => todo!()



            }

        }
        _ => {
            //didn't process the message.
            LRESULT(1)
        }
    }
}

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
        MOUSE_STATE.get_or_init(Mutex::default).lock().unwrap().register_coalesced(shared);
        PlatformCoalescedMouse {

        }
    }
}
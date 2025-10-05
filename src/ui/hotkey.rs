use crate::core::policy::Policy;
use winapi::um::winuser::*;
use std::ptr;

pub struct HotkeyListener {
    policy: Policy,
}

impl HotkeyListener {
    pub fn new(policy: Policy) -> Self { Self { policy } }

    pub fn run_event_loop(&mut self) {
        unsafe {
            RegisterHotKey(ptr::null_mut(), 1, 0, VK_SNAPSHOT as u32);
            log::info!("Hotkey registered (PrintScreen)");
            let mut msg: MSG = std::mem::zeroed();

            loop {
                if GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
                    if msg.message == WM_HOTKEY {
                        if msg.wParam == 1 {
                            log::info!("PrintScreen pressed → capturing");
                            if let Some(img) = crate::core::capture::capture_fullscreen() {
                                crate::export::flatten::export_and_copy(&img);
                            } else {
                                log::error!("Capture failed");
                            }
                        }
                    }
                }
            }
        }
    }
}

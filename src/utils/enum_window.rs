#![allow(dead_code)]

use tracing::{debug, error, info, trace, warn};
use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

pub extern "system" fn enum_window(hwnd: HWND) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(hwnd, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        let mut info = WINDOWINFO {
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        GetWindowInfo(hwnd, &mut info).unwrap();

        if !text.is_empty() && info.dwStyle.contains(WS_VISIBLE) {
            println!(
                "text: {} (left: {}, top: {})",
                text, info.rcWindow.left, info.rcWindow.top
            );
        }

        true.into()
    }
}

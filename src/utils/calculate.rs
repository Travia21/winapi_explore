use tracing::{debug, error, info, trace, warn};

use std::ptr::null_mut;
use std::slice;

use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, SIZE},
        Graphics::Gdi::*,
        UI::WindowsAndMessaging::{GetDlgItemTextW, GetWindowTextW},
    },
};

pub unsafe fn get_text_dimensions(
    parent_hwnd: HWND,
    edit_ctrl_id: i32,
) -> SIZE {
    let device_context: HDC = GetDC(parent_hwnd);
    debug!("device_context: {:#?}", device_context);
    //test for null return (failure)

    let mut text_size: windows::Win32::Foundation::SIZE = Default::default();
    if device_context.0 == null_mut() {
        return text_size; // return (0,0)
    }

    /* This gets the text from the edit_control */
    let mut buffer: Vec<u16> = vec![0; 128];
    let buffer_length = GetDlgItemTextW(parent_hwnd, edit_ctrl_id, &mut buffer);
    debug!("Text length: {:?}", buffer_length);
    debug!("Text: {:?}", String::from_utf16(&buffer).unwrap());

    /* TODO:
     * Each line must be measured individually and added together
     */
    GetTextExtentPoint32W(device_context, &mut buffer, &mut text_size);
    ReleaseDC(parent_hwnd, device_context);
    info!("Text size:\n{:#?}", text_size);

    text_size
}

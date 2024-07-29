use std::ptr::null_mut;
use std::slice;

use windows::{
    core::*,
    Win32::{
        Foundation::{ HWND, SIZE },
        Graphics::Gdi::*,
        UI::WindowsAndMessaging::{ GetDlgItemTextW, GetWindowTextW }
    }
};

pub unsafe fn get_text_dimensions(hwnd: HWND) -> SIZE {
    let mut text_size: windows::Win32::Foundation::SIZE = SIZE {
        cx: 0,
        cy: 0,
    };

    let device_context: HDC = GetDC(hwnd);
    //test for null return (failure)
    if device_context.0 == null_mut() {
        return text_size; // return (0,0)
    }

    /* This gets the text from the edit_control */
    let text: &mut [u16] = &mut [];
    GetDlgItemTextW(hwnd, 1, text);

    let something: Vec<u16> =
        "This is a really long string that won't fit in the default edit_control dimensions."
        .encode_utf16()
        .collect();
    let s_ptr: &[u16] = slice::from_raw_parts(something.as_ptr(), something.len());

    //GetTextExtentPoint32W(device_context, text, &mut text_size as *mut _);
    GetTextExtentPoint32W(device_context, s_ptr, &mut text_size as *mut _);
    println!("\n\nText size: {:#?}", text_size);
    println!("\n\nText: {:#?}", text);

    text_size
}

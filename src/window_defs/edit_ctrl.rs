#![allow(dead_code)]
#![allow(unused_variables)]

use std::ffi::c_void;
use std::mem;
use std::ptr::null_mut;
use tracing::{debug, error, info, trace, warn};
use windows::{
    core::*,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::*,
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
};

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM)
-> LRESULT {
    match msg {
        WM_CREATE => {
            error!("find me");
        },
        WM_COMMAND => {
            trace!("Edit_Ctrl received WM_COMMAND");
        }
        WM_NCHITTEST => {
            trace!("Edit_Ctrl received WM_NCHITTEST");
        }
        WM_PAINT => {
            trace!("Edit_Ctrl received WM_PAINT");
            _ = ValidateRect(hwnd, None);
        }
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    LRESULT(0)
}

pub unsafe fn build_window(parent_hwnd: HWND) -> HWND {
    /*
     * Create the Edit Control window
     */
    let h_instance: HINSTANCE = GetModuleHandleW(None)
        .expect("Failed to get module handle")
        .into();
    let class_name = HSTRING::from("edit_ctrl");

    let edit_ctrl_text = HSTRING::from("Initial text");
    //let edit_ctrl_width = 280;

    /* I need a `*mut RECT` receptacle in memory */
    let mut client_rect: RECT = RECT { 
        left: 0,
        right: 0,
        top: 0,
        bottom: 0
    };
    GetClientRect(parent_hwnd, &mut client_rect);

    //let client_rect = *client_rect;
    //let edit_ctrl_width = 200;
    let edit_ctrl_width = client_rect.right - client_rect.left - 20;
    //let edit_ctrl_height = 25;
    let edit_ctrl_height = client_rect.bottom - client_rect.top - 20;
    println!("ec\nwidth: {} x height: {}", edit_ctrl_width, edit_ctrl_height);

    /* Experimenting with custom window class */
    /*
    let window_class = WNDCLASSW {
        style: WINDOW_STYLE(ES_MULTILINE | ES_LEFT),
        lpfnWndProc: Some(wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,w
        hInstance: h_instance,
        //hIcon: LoadIconW(None, IDI_SHIELD).expect("Failed to LoadIconW"),
        hIcon: ,
        hCursor: LoadCursorW(None, IDC_ARROW).expect("Failed to LoadCursorW"),
        hbrBackground: GetSysColorBrush(COLOR_HOTLIGHT),
        lpszMenuName: PCWSTR(HSTRING::from("edit_menu").as_ptr()),
        lpszClassName: PCWSTR(class_name.as_ptr()),
    };
    let _atom = RegisterClassW(&window_class);
    */

    /* Doesn't display correctly initially */
    let edit_ctrl_hwnd = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        //PCWSTR(class_name.as_ptr()),
        w!("EDIT"), // class name
        PCWSTR(edit_ctrl_text.as_ptr()),
        WS_CHILD | WS_VISIBLE | WS_BORDER
        | WINDOW_STYLE((ES_MULTILINE | ES_LEFT) as u32), //ES_MULTILINE = Edit Control
        10,
        10,
        edit_ctrl_width,
        edit_ctrl_height,
        parent_hwnd, // where to put the edit control
        None,
        h_instance,
        Some(null_mut()),
    ).expect("Failed to create initial WindowExW");

    InvalidateRect(edit_ctrl_hwnd, None, true);
    UpdateWindow(edit_ctrl_hwnd);

    trace!("Initial CreateWindowExW\n{:#?}", edit_ctrl_hwnd);

    edit_ctrl_hwnd
}

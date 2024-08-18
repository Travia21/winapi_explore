#![allow(unused_imports)]
#![allow(dead_code)]

use crate::utils::calculate;
use crate::utils::{calculate::get_text_dimensions, window_messages::message_to_string};
use crate::window_defs::edit_ctrl;

use std::borrow::BorrowMut;
use std::fs::File;
use std::io::prelude::*;
use std::ops::DivAssign;
use std::path::PathBuf;
use std::time::{Instant, UNIX_EPOCH};

use std::ffi::c_void;
use std::mem;
use std::ptr::null_mut;
use tracing::{debug, error, info, trace, warn};
use windows::{
    core::*,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::*,
        System::{LibraryLoader::GetModuleHandleW, SystemServices::SS_NOTIFY},
        UI::WindowsAndMessaging::*,
    },
};

const EDIT_CTRL_ID: i32 = 101;
const EDIT_CONT_ID: i32 = 102;
const BUTTON1_ID: i32 = 201;
const BUTTON2_ID: i32 = 202;
const BUTTON3_ID: i32 = 203;
const BUTTON4_ID: i32 = 204;

/// `GetDlgItem()` requires the HWND to be the immediate parent of the window identifier.
pub unsafe fn get_edit_ctrl_handle(main_window_hwnd: HWND) -> HWND {
    let container_hwnd =
        GetDlgItem(main_window_hwnd, EDIT_CONT_ID).expect("Failed to get container handle");

    GetDlgItem(container_hwnd, EDIT_CTRL_ID).expect("Failed to get edit_ctrl handle")
}

fn read_text_from_file(filename: Option<&str>) -> Result<String> {
    trace!("File: {}", filename.unwrap()); //placeholder

    debug!("Here");
    let file = match filename {
        Some(filename) => File::open(filename),
        None => {
            let location = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("resources")
                .join("strings.txt");
            File::open(location)
        }
    };
    debug!("File path: {:#?}", file);

    match file {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            //content = content.replace("\n", "\r\n");

            debug!("File content:\n{}", content);
            Ok(content)
        }
        Err(error) => Ok(format!("Error: {:#?}\nHello, world", error)),
    }
}

fn pop_up(hwnd: HWND) -> Result<()> {
    trace!("{:#?}", hwnd);
    Ok(())
}

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    /*
     * TODO:
     * Parse the l and w param fields for each message type.
     */
    /*debug!("Message received.\nmessage: {}\nwparam: {:#?}\nlparam: {:#?}",
        message_to_string(msg),
        wparam,
        lparam
    );
    */
    /*
     * TODO:
     * Every three seconds, record each message received and how many times it was received, then
     * print to screen.
     */

    match msg {
        WM_CREATE => {
            println!("Primary -> WM_CREATE");

            let h_instance: HINSTANCE = GetModuleHandleW(None)
                .expect("Failed to get module handle")
                .into();
            let btn_class_name = HSTRING::from("BUTTON");
            let btn_y_pos: i32 = 50;

            // Button 1
            CreateWindowExW(
                WINDOW_EX_STYLE(0_u32),
                PCWSTR(btn_class_name.as_ptr()),
                PCWSTR(HSTRING::from("Button 1").as_ptr()),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                10,
                btn_y_pos, // x,y pos
                60,
                25, // w,h dimensions
                hwnd,
                HMENU(BUTTON1_ID as *mut c_void),
                h_instance,
                None,
            )
            .unwrap();

            // Button 2
            CreateWindowExW(
                WINDOW_EX_STYLE(0_u32),
                PCWSTR(btn_class_name.as_ptr()),
                PCWSTR(HSTRING::from("Button 2").as_ptr()),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                80,
                btn_y_pos, // x,y pos
                60,
                25, // w,h dimensions
                hwnd,
                HMENU(BUTTON2_ID as *mut c_void),
                h_instance,
                None,
            )
            .unwrap();

            // Button 3
            CreateWindowExW(
                WINDOW_EX_STYLE(0_u32),
                PCWSTR(btn_class_name.as_ptr()),
                PCWSTR(HSTRING::from("Button 3").as_ptr()),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                150,
                btn_y_pos, // x,y pos
                60,
                25, // w,h dimensions
                hwnd,
                HMENU(BUTTON3_ID as *mut c_void),
                h_instance,
                None,
            )
            .unwrap();

            // Button 4
            CreateWindowExW(
                WINDOW_EX_STYLE(0_u32),
                PCWSTR(btn_class_name.as_ptr()),
                PCWSTR(HSTRING::from("Button 4").as_ptr()),
                WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_PUSHBUTTON as u32),
                220,
                btn_y_pos, // x,y pos
                60,
                25, // w,h dimensions
                hwnd,
                HMENU(BUTTON4_ID as *mut c_void),
                h_instance,
                None,
            )
            .unwrap();

            InvalidateRect(hwnd, None, true);
            UpdateWindow(hwnd);
        }
        WM_COMMAND => {
            trace!("Primary -> WM_COMMAND");
            //match LOWORD(wparam as DWORD)
            match wparam.0 as i32 & 0xffff {
                BUTTON1_ID => {
                    info!("Button 1 here");
                    SendMessageW(hwnd, WM_PAINT, WPARAM(0), LPARAM(0));
                }
                BUTTON2_ID => {
                    info!("Button 2 here");
                    let text = match read_text_from_file(None) {
                        Ok(text) => text,
                        Err(error) => panic!("rtff error: {:#?}", error),
                    };
                    adjust_edit_ctrl(hwnd, Some(&text));
                }
                BUTTON3_ID => {
                    info!("Button 3 here");
                    let window_style =
                        WINDOW_STYLE(GetWindowLongW(get_edit_ctrl_handle(hwnd), GWL_STYLE) as u32);
                    debug!("window style: {:#?}", window_style.0);
                    adjust_edit_ctrl(hwnd, Some(&format!("Edit control ID: {:#?}", window_style)));
                }
                BUTTON4_ID => {
                    println!("try to destroy");
                    SendMessageW(hwnd, WM_DESTROY, WPARAM(0), LPARAM(0));
                }
                _ => (),
            }

            return LRESULT(0); //placeholder
        }
        WM_PAINT => {
            //InvalidateRect(hwnd, None, true);
            //UpdateWindow(hwnd);
            DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        // TODO: 
        // Only seems to work after moving the window...
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    LRESULT(0)
}

/**
 * Sets the text of the edit control.
 */
pub fn adjust_edit_ctrl(
    hwnd: HWND,
    text: Option<&str>,
) -> Result<()> {
    unsafe {
        let edit_ctrl_hwnd = get_edit_ctrl_handle(hwnd);

        let mut info: WINDOWINFO = Default::default();
        GetWindowInfo(edit_ctrl_hwnd, &mut info);

        let cur_width = info.rcWindow.right - info.rcWindow.left;
        let cur_height = info.rcWindow.bottom - info.rcWindow.top;

        let text: String = match text {
            Some(input) => String::from(input),
            //None => read_text_from_file("resources/strings.txt").expect("Error reading file"),
            None => {
                let output = format!(
                    "w: {} x h: {}",
                    cur_width.to_string(),
                    cur_height.to_string()
                );
                output
            }
        };

        let text = HSTRING::from(text);
        match SetWindowTextW(edit_ctrl_hwnd, PCWSTR(text.as_ptr())) {
            Ok(_) => {
                info!("Successfully set window text");
            }
            Err(error) => {
                error!("Error setting window text.\n{}", error);
                return Err(error);
            }
        }

        let new_width = 200;
        let new_height =
            calculate::get_text_dimensions(GetDlgItem(hwnd, EDIT_CONT_ID)?, EDIT_CTRL_ID).cy;
        /*
         * TODO: Set edit_ctrl dimensions
         * SetWindowPos (stupid name for a resizing function)
         */
        error!("{} x {}", new_width, new_height);
        SetWindowPos(
            edit_ctrl_hwnd,
            HWND_TOP,
            info.rcWindow.left, // x
            info.rcWindow.top,  // y
            new_width,
            new_height,
            SWP_NOMOVE | SWP_SHOWWINDOW,
        )
        .into()
    }
}

// This function should be moved to edit_ctrl.rs
unsafe fn build_edit_ctrl_container(parent_hwnd: HWND) -> HWND {
    let mut client_rect: RECT = Default::default();
    GetClientRect(parent_hwnd, &mut client_rect);
    let container_width = client_rect.right - client_rect.left;

    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("STATIC"),
        None,
        WS_CHILD | WS_VISIBLE | WINDOW_STYLE(SS_NOTIFY.0),
        0,               // x
        0,               // y
        container_width, // width
        50,              // height
        parent_hwnd,
        HMENU(EDIT_CONT_ID as *mut c_void),
        None,
        Some(null_mut()),
    )
    .expect("Failed to create edit control container")
}

pub fn build_window(
    class_name: &str,
    window_name: &str,
) -> HWND {
    unsafe {
        let h_instance = HINSTANCE(
            GetModuleHandleW(None)
                .expect("Failed to get module handle")
                .0,
        );
        let class_name_h = HSTRING::from(class_name);
        let window_name_h = HSTRING::from(window_name);

        let window_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: LoadIconW(None, IDI_APPLICATION).expect("Failed to LoadIconW"),
            hCursor: LoadCursorW(None, IDC_ARROW).expect("Failed to LoadCursorW"),
            hbrBackground: HBRUSH((COLOR_BTNFACE.0 + 1) as _), // This is obtuse, the +1 is
                                                               // required
            lpszMenuName: PCWSTR(HSTRING::from("primary_menu").as_ptr()),
            lpszClassName: PCWSTR(class_name_h.as_ptr()),
        };

        let _atom = RegisterClassW(&window_class);

        // Primary OverlappedWindow
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0_u32),
            PCWSTR(class_name_h.as_ptr()),
            PCWSTR(window_name_h.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            0,
            0, // x,y pos
            320,
            150, // w,h dimensions
            None,
            None,
            h_instance,
            None,
        )
        .expect("Failed to create window");

        // This function should be moved to edit_ctrl.rs
        let container_hwnd = build_edit_ctrl_container(hwnd);
        let _edit_ctrl_hwnd = edit_ctrl::build_window(container_hwnd, EDIT_CTRL_ID);

        get_text_dimensions(container_hwnd, EDIT_CTRL_ID);
        hwnd
    }
}

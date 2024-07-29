#![allow(unused_imports)]
#![allow(dead_code)]

use crate::utils::{calculate::get_text_dimensions, window_messages::message_to_string};
use crate::window_defs::edit_ctrl;

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
const BUTTON1_ID: i32 = 201;
const BUTTON2_ID: i32 = 202;
const BUTTON3_ID: i32 = 203;
const BUTTON4_ID: i32 = 204;

//These are wrong
// TODO: Figure out render width of characters in the edit_ctrl
const CHARACTER_WIDTH: i32 = 8; // Estimated average character width
const MAX_WIDTH: i32 = 768; // Maximum width for the text field

pub fn get_edit_ctrl_handle(main_window_hwnd: HWND) -> HWND {
    unsafe {
        match GetWindow(main_window_hwnd, GW_CHILD) {
            Ok(handle) => handle,
            Err(error) => panic!("{error}"),
        }
    }
}

fn read_text_from_file(filename: &str) -> Result<String> {
    trace!("{filename}"); //placeholder
    Ok(String::from("Hello, world!"))
}

fn calculate_text_height(
    text: &str,
    width: i32,
) -> i32 {
    trace!("{text}"); //placeholder
    32_i32 * width
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
            ).unwrap();

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
            ).unwrap();

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
            ).unwrap();

            let h4 = CreateWindowExW(
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
            ).unwrap();

            InvalidateRect(h4, None, true);
            InvalidateRect(hwnd, None, true);
        }
        WM_COMMAND => {
            println!("Primary -> WM_COMMAND");
            //match LOWORD(wparam as DWORD)
            match wparam.0 as i32 & 0xffff {
                BUTTON1_ID => pop_up(hwnd),
                BUTTON2_ID => adjust_edit_ctrl(hwnd, None),
                BUTTON3_ID => pop_up(hwnd),
                BUTTON4_ID => pop_up(hwnd),
                _ => Ok(()),
            }
            .expect("Some WM_COMMAND failure");

            return LRESULT(0); //placeholder
        }
        WM_PAINT => {
            trace!("Primary -> WM_PAINT");
            _ = ValidateRect(hwnd, None);
        }
        WM_DESTROY => PostQuitMessage(0),
        _ => {
            let h4 = GetDlgItem(hwnd, BUTTON4_ID).unwrap();
            InvalidateRect(hwnd, None, true);
            InvalidateRect(h4, None, true);
            //UpdateWindow(hwnd);

            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
    LRESULT(0)
}

pub fn adjust_edit_ctrl(
    hwnd: HWND,
    text: Option<&str>,
) -> Result<()> {
    unsafe {
        let edit_ctrl_hwnd = match GetDlgItem(hwnd, EDIT_CTRL_ID) {
            Ok(handle) => handle,
            Err(error) => {
                error!("Error getting edit ctrl handle.\n{}", error);
                return Err(error);
            }
        };

        let mut info = WINDOWINFO {
            // out var
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        GetWindowInfo(edit_ctrl_hwnd, &mut info);
        let _cur_width = info.rcWindow.right - info.rcWindow.left;
        let _cur_height = info.rcWindow.bottom - info.rcWindow.top;

        let text: String = match text {
            Some(input) => String::from(input),
            //None => read_text_from_file("resources/strings.txt").expect("Error reading file"),
            None => String::from(format!(
                "w: {} x h: {}",
                _cur_width.to_string(),
                _cur_height.to_string()
            )),
        };
        //let new_width = (text.chars().count() as i32 * CHARACTER_WIDTH).min(MAX_WIDTH);
        let new_width = 200;
        let new_height = calculate_text_height(&text, new_width);
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

        /*
         * TODO: Set edit_ctrl dimensions
         * SetWindowPos (stupid name for a resizing function)
         */
        error!("{} x {}", new_width, new_height);
        SetWindowPos(
            edit_ctrl_hwnd,
            //HWND_NOTOPMOST, doesn't work
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

unsafe fn build_edit_ctrl_container(parent_hwnd: HWND) -> HWND {
    // Get dimensions
    let mut client_rect: RECT = RECT {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    GetClientRect(parent_hwnd, &mut client_rect);

    println!("{:#?}", SS_NOTIFY.0);
    CreateWindowExW(
        WINDOW_EX_STYLE(0),
        w!("STATIC"),
        None,
        WS_CHILD | WS_VISIBLE | WINDOW_STYLE(SS_NOTIFY.0),
        0,
        0, //x,y pos
        300,
        50, //w,h dimensions
        parent_hwnd,
        None,
        None,
        Some(null_mut()),
    )
    .expect("Failed to create container")
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
            hbrBackground: HBRUSH((COLOR_BACKGROUND.0 + 1) as _), // This is obtuse
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
            //CW_USEDEFAULT, CW_USEDEFAULT, // x,y pos
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

        let container_hwnd = build_edit_ctrl_container(hwnd);
        let edit_ctrl_hwnd = edit_ctrl::build_window(container_hwnd);

        get_text_dimensions(edit_ctrl_hwnd);
        hwnd
    }
}

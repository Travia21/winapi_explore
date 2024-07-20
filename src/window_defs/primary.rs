#![allow(unused_imports)]

use crate::utils::window_messages::message_to_string;

use std::ffi::c_void;
use std::mem;
use std::ptr::null_mut;
use tracing::{error, warn, debug, info, trace};
use windows::{
    core::*,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::*,
        System::LibraryLoader::GetModuleHandleW,
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

pub fn adjust_edit_ctrl(hwnd: HWND, text: Option<&str>) -> Result<()> {
    let text: String = match text {
        Some(input) => String::from(input),
        None => read_text_from_file("resources/strings.txt").expect("Error reading file")
    };
    let width = (text.chars().count() as i32 * CHARACTER_WIDTH).min(MAX_WIDTH);
    let height = calculate_text_height(&text, width);

    warn!("width: {width} x height: {height}"); // placeholder

    unsafe {
        let text = HSTRING::from(text);
        let edit_ctrl_hwnd = match GetDlgItem(hwnd, EDIT_CTRL_ID) {
            Ok(handle) => handle,
            Err(error) => {
                error!("Error getting edit ctrl handle.\n{}", error);
                return Err(error);
            }
        };

        match SetWindowTextW(edit_ctrl_hwnd, PCWSTR(text.as_ptr())) {
            Ok(_) => {
                trace!("Successfully set window text");
                Ok(())
            }
            Err(error) => {
                error!("Error setting window text.\n{}", error);
                Err(error)
            }
        }

        /*
         * TODO: Set edit_ctrl dimensions
         */
        
    }
}

pub fn get_edit_ctrl_handle(main_window_hwnd: HWND) -> HWND {
    unsafe {
        match GetWindow(main_window_hwnd, GW_CHILD) {
            Ok(handle) => handle,
            Err(error) => panic!("{error}")
        }
    }
}

fn read_text_from_file(filename: &str) -> Result<String> {
    trace!("{filename}"); //placeholder
    Ok(String::from("Hello, world!"))
}

fn calculate_text_height(text: &str, width: i32) -> i32 {
    trace!("{text}"); //placeholder
    32_i32 * width
}

fn pop_up(hwnd: HWND) -> Result<()> {
    trace!("{:#?}", hwnd);
    Ok(())
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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
        WM_CREATE => create_window(hwnd),
        WM_COMMAND => {
            trace!("Window received WM_COMMAND");
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
        WM_NCHITTEST => {
            trace!("Window received WM_NCHITTEST");
        }
        WM_PAINT => {
            trace!("Window received WM_PAINT");
            _ = ValidateRect(hwnd, None);
        }
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    LRESULT(0)
}

fn create_window(hwnd: HWND) {
    unsafe {
        let h_instance: HINSTANCE = GetModuleHandleW(None)
            .expect("Failed to get module handle")
            .into();
        let edit_ctrl_text = HSTRING::from("Initial text");

        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            w!("EDIT"),
            PCWSTR(edit_ctrl_text.as_ptr()),
            WS_CHILD | WS_VISIBLE | WS_BORDER | WINDOW_STYLE(ES_LEFT as u32),
            10,
            10,
            280,
            25,
            hwnd,
            HMENU(EDIT_CTRL_ID as *mut c_void),
            h_instance,
            Some(null_mut()),
        )
        .expect("Failed to create initial WindowExW");
    }
}

pub fn init(class_name: &str, window_name: &str) -> HWND {
    unsafe {
        let h_instance = HINSTANCE(
            GetModuleHandleW(None)
                .expect("Failed to get module handle")
                .0,
        );
        let class_name = HSTRING::from(class_name);
        let window_name = HSTRING::from(window_name);

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
            lpszClassName: PCWSTR(class_name.as_ptr()),
        };

        let atom = RegisterClassW(&window_class);
        debug!("RegisterClassW return code: {:#?}", atom);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE(0_u32),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(window_name.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            300,
            100,
            None,
            None,
            h_instance,
            None,
        )
        .expect("Failed to create window");
        debug!("Initial CreateWindowExW\n{:#?}", hwnd);

        adjust_edit_ctrl(hwnd, None);

        hwnd
    }
}

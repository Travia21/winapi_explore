#![allow(unused_must_use)]
#![allow(unused_imports)]

mod utils;
mod window_defs;

use utils::enum_window;
use utils::window_messages::message_to_string;
use window_defs::primary;

use std::time::{Duration, Instant};
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::FmtSubscriber;
use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/*
 * TODO:
 * A drop down for each window handle currently running in the OS. The edit_ctrl then starts
 * printing that handle's Window Messages.
 *
 * TODO:
 * Hook chains
 * A function that intercepts a particular type of event is called a hook procedure. A hook
 * procedure can act on each event it receives, then modify or discard the event.
 */
fn main() {
    // Tracing setup
    // Level::OFF < ERROR < WARN < DEBUG = INFO < TRACE
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Set tracing default subscriber failed.");

    let hwnd = primary::build_window("class_name", "Some window");

    // Message loop
    let mut msg: MSG = unsafe { std::mem::zeroed() };
    unsafe {
        let mut timer = Instant::now();
        while IsWindow(hwnd).into() && GetMessageW(&mut msg, hwnd, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);

            //error!("\n\nmsg: {}", message_to_string(msg.message));

            if timer.elapsed() >= Duration::from_secs(1) {
                timer = Instant::now();
                primary::adjust_edit_ctrl(hwnd, None);
                /*
                primary::adjust_edit_ctrl(hwnd, Some(message_to_string(msg.message)))
                    .expect("Failed to adjust text");
                */
                //enum_window::enum_window(primary::get_edit_ctrl_handle(hwnd));
            }
        }
    }
}

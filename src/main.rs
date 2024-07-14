#![allow(unused_must_use)]
#![allow(unused_imports)]

mod window_defs;
mod utils;

use window_defs::primary;
use utils::window_messages::message_to_string;

use tracing::{ info, Level };
use tracing_subscriber::FmtSubscriber;
use windows::core::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use std::time::{ Duration, Instant };

fn main() {
    // Tracing setup
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Set tracing default subscriber failed.");

    let hwnd = primary::init("class_name", "Some window");

    // Message loop
    let mut msg: MSG = unsafe { std::mem::zeroed() };
    unsafe {
        let mut timer = Instant::now();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if timer.elapsed() >= Duration::from_secs(1) {
                timer = Instant::now();
                primary::adjust_edit_ctrl(hwnd).expect("Failed to adjust text");
            }
        }
    }
}

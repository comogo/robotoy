#![allow(warnings)]
extern crate sdl2;

mod controller;
mod rf24;

use controller::Controller;
use rf24::{DataRate, PowerLevel, Radio};
use std::thread::sleep;
use std::time::Duration;

fn test_radio() {
    let mut controller: Controller = controller::init();
    let mut radio = Radio::new("aaaaa", DataRate::_250Kbps, PowerLevel::_0dBm, 125, 25).unwrap();

    println!("Initialized!");

    radio.configure().unwrap();
    radio.print_rf_details();

    'running: loop {
        let should_continue = controller.update_state();

        if should_continue == false {
            break 'running;
        }

        let state = controller.get_state();
        radio.send(&state.to_bytes()).unwrap();

        // sleep(Duration::from_millis(1000));
    }

    println!("Shutdown!");
}

fn _test_rf() {
    let radio = Radio::new("aaaaa", DataRate::_250Kbps, PowerLevel::_0dBm, 125, 25).unwrap();
}

pub fn main() {
    test_radio();
}

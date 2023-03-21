#![allow(warnings)]
extern crate sdl2;

mod controller;
mod rf24;

use controller::Controller;
use rf24::{DataRate, PowerLevel, Radio};
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    let channel: u8 = 125;
    let ce_pin: u8 = 25;
    let address: &str = "aaaaa";
    let data_rate: DataRate = DataRate::_250Kbps;
    let power_level: PowerLevel = PowerLevel::_0dBm;

    let mut controller: Controller = controller::init();
    let mut radio: Radio = Radio::new(address, data_rate, power_level, channel, ce_pin).unwrap();

    println!("Starting up");

    radio.configure().unwrap();
    radio.print_rf_details();

    'running: loop {
        let should_continue = controller.update_state();

        if should_continue == false {
            break 'running;
        }

        let state = controller.get_state();
        radio.send(&state.to_bytes()).unwrap();
    }
}

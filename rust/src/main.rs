#![allow(warnings)]
extern crate sdl2;

mod controller;
mod radio;
mod registers;
mod rf24;

use controller::Controller;
use radio::Radio;
use std::thread::sleep;
use std::time::Duration;

fn test_radio() {
    // let mut controller: Controller = controller::init();
    let mut radio = Radio::new(25).unwrap();
    let rx_address: [u8; 5] = *b"aaaaa";
    let tx_address: [u8; 5] = *b"aaaaa";

    println!("Initialized!");

    radio.setup().unwrap();
    radio.set_address(tx_address, rx_address).unwrap();
    radio.set_channel(125).unwrap();
    radio.stop_listening().unwrap();
    radio.print_details().unwrap();
    radio.send(&[5u8]).unwrap();

    // 'running: loop {
    //     let should_continue = controller.update_state();

    //     if should_continue == false {
    //         break 'running;
    //     }

    //     let state = controller.get_state();
    //     radio.send(&state.to_bytes()).unwrap();
    //     radio.send(&[5u8]).unwrap();

    //     sleep(Duration::from_millis(1000));
    // }

    println!("Shutdown!");
}

fn test_rf() {
    let rf = rf24::RF24::new().unwrap();
    rf.test().unwrap();
}

pub fn main() {
    test_rf();
}

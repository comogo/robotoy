extern crate sdl2;

mod controller;
mod radio;

use controller::Controller;
use radio::Radio;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn scan(mut radio: Radio) {
    radio.set_rx_mode().unwrap();
    radio.standby();
    for channel in 0..126 {
        radio.set_channel(channel).unwrap();
        radio.listen().unwrap();
        sleep(Duration::from_micros(300));
        let rpd = radio.received_power_detector().unwrap();
        radio.stop_listening().unwrap();
        println!("Channel {}: {}", channel, if rpd { 1 } else { 0 });
    }
    radio.standby();
}

pub fn main() {
    // let mut controller: Controller = controller::init();
    // let mut initial_time = Instant::now();
    let mut radio = Radio::new(25).unwrap();
    let rx_address: [u8; 5] = *b"from1";
    let tx_address: [u8; 5] = *b"toDev";

    println!("Initialized!");

    radio.setup().unwrap();
    radio.set_address(tx_address, rx_address).unwrap();
    radio.set_channel(100).unwrap();
    radio.set_tx_mode().unwrap();
    scan(radio);

    // 'running: loop {
    //     let should_continue = controller.update_state();

    //     if should_continue == false {
    //         break 'running;
    //     }

    //     let elapsed = initial_time.elapsed().as_millis();

    //     if elapsed > 1000 {
    //         println!("{:?}", controller.get_state());
    //         initial_time = Instant::now();
    //     }
    // }

    println!("Shutdown!");
}

extern crate sdl2;

mod controller;
mod radio;

use controller::Controller;
use radio::Radio;
use std::time::Instant;

pub fn main() {
    let mut controller: Controller = controller::init();
    let mut initial_time = Instant::now();
    let radio = Radio::new(10, 9).unwrap();

    println!("Initialized!");

    radio.setup().unwrap();

    'running: loop {
        let should_continue = controller.update_state();

        if should_continue == false {
            break 'running;
        }

        let elapsed = initial_time.elapsed().as_millis();

        if elapsed > 1000 {
            println!("{:?}", controller.get_state());
            initial_time = Instant::now();
        }
    }

    println!("Shutdown!");
}

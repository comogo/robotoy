extern crate sdl2;

mod controller;

use controller::ControllerState;
use std::sync::{Arc, Mutex};
use sdl2::{
    controller::GameController,
    event::Event,
};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let controller_subsystem = sdl_context.game_controller().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let controller_state = Arc::new(Mutex::new(ControllerState::new()));
    let mut controller: Option<GameController> = None;
    let running = Arc::new(Mutex::new(true));

    println!("Initialized!");

    let thread_running = running.clone();
    let thread_controller_state = controller_state.clone();
    let debug_info= std::thread::spawn(move || loop {
        if !*thread_running.lock().unwrap() {
            println!("Shutting down debug thread...");
            break;
        }
        println!("{:?}", thread_controller_state.lock().unwrap());
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    let main_running = running.clone();
    let main_controller_state = controller_state.clone();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    let mut running = main_running.lock().unwrap();
                    *running = false;
                    break 'running
                },
                Event::ControllerDeviceAdded { which, .. } => {
                    if controller_subsystem.num_joysticks().unwrap() > 1 {
                        println!("More than one controller attached. Only one can be used at a time");
                    } else {
                        let new_controller = controller_subsystem.open(which).unwrap();
                        println!("Controller attached: {}", new_controller.name());
                        controller = Some(new_controller);
                    }
                },
                Event::ControllerDeviceRemoved { which, .. } => {
                    controller = None;
                    println!("Joystick detached: {}", which);
                },
                Event::ControllerAxisMotion { axis, value, .. } => {
                    let mut controller_state = main_controller_state.lock().unwrap();
                    controller_state.update_axis(axis, value);
                },
                Event::ControllerButtonDown { button, ..} => {
                    let mut controller_state = main_controller_state.lock().unwrap();
                    controller_state.update_button(button, true);
                },
                Event::ControllerButtonUp { button, .. } => {
                    let mut controller_state = main_controller_state.lock().unwrap();
                    controller_state.update_button(button, false);
                },
                _ => {}
            }
        }
    }

    let c = controller.unwrap();
    drop(c);
    debug_info.join().unwrap();
    println!("Shutdown!");
}

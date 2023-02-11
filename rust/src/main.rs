extern crate sdl2;

mod controller;

use controller::ControllerState;
use sdl2::{
    controller::GameController,
    event::Event,
};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let controller_subsystem = sdl_context.game_controller().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut controller_state = ControllerState::new();
    let mut controller: Option<GameController> = None;

    println!("Initialized!");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
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
                    controller_state.update_axis(axis, value);
                },
                Event::ControllerButtonDown { button, ..} => {
                    controller_state.update_button(button, true);
                },
                Event::ControllerButtonUp { button, .. } => {
                    controller_state.update_button(button, false);
                },
                _ => {}
            }
        }
    }

    let c = controller.unwrap();
    drop(c);
    println!("Shutdown!");
}

extern crate sdl2;

use std::sync::{Arc, Mutex};
use sdl2::{
    controller::{Button, GameController, Axis},
    event::Event,
};

const JOYSTICK_DEADZONE: i16 = 4000;

#[derive(Debug)]
struct ControllerState {
    x: bool,
    circle: bool,
    square: bool,
    triangle: bool,
    select: bool,
    start: bool,
    l1: bool,
    r1: bool,
    l2: u16,
    r2: u16,
    yaw: i16,
    throttle: i16,
    pitch: i16,
    roll: i16
}

impl ControllerState {
    fn new() -> Self {
        ControllerState {
            x: false,
            circle: false,
            square: false,
            triangle: false,
            select: false,
            start: false,
            l1: false,
            r1: false,
            l2: 0,
            r2: 0,
            yaw: 0,
            throttle: 0,
            pitch: 0,
            roll: 0
        }
    }

    pub fn update_button(&mut self, button: Button, value: bool) {
        match button {
            Button::A => self.x = value,
            Button::B => self.circle = value,
            Button::X => self.square = value,
            Button::Y => self.triangle = value,
            Button::Back => self.select = value,
            Button::Start => self.start = value,
            Button::LeftShoulder => self.l1 = value,
            Button::RightShoulder => self.r1 = value,
            _ => {}
        }
    }

    pub fn update_axis(&mut self, axis: Axis, value: i16) {
        match axis {
            Axis::LeftX => self.yaw = self.calculate_axis_value_with_deadzone(value),
            Axis::LeftY => self.throttle = self.calculate_axis_value_with_deadzone(value),
            Axis::RightX => self.roll = self.calculate_axis_value_with_deadzone(value),
            Axis::RightY => self.pitch = self.calculate_axis_value_with_deadzone(value),
            Axis::TriggerLeft => self.l2 = value as u16,
            Axis::TriggerRight => self.r2 = value as u16,
        }
    }

    fn calculate_axis_value_with_deadzone(&self, value: i16) -> i16 {
        if value < -JOYSTICK_DEADZONE || value > JOYSTICK_DEADZONE {
            value
        } else {
            0
        }
    }
}

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

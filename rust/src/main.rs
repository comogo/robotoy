extern crate sdl2;

use sdl2::event::Event;

const JOYSTICK_DEADZONE: i16 = 8000;
const JOYSTICK_BUTTON_X: u8 = 0;
const JOYSTICK_BUTTON_CIRCLE: u8 = 1;
const JOYSTICK_BUTTON_SQUARE: u8 = 2;
const JOYSTICK_BUTTON_TRIANGLE: u8 = 3;
const JOYSTICK_BUTTON_SELECT: u8 = 4;
const JOYSTICK_BUTTON_START: u8 = 6;
const JOYSTICK_BUTTON_L3: u8 = 7;
const JOYSTICK_BUTTON_R3: u8 = 8;
const JOYSTICK_BUTTON_L1: u8 = 9;
const JOYSTICK_BUTTON_R1: u8 = 10;
const JOYSTICK_BUTTON_ARROW_UP: u8 = 11;
const JOYSTICK_BUTTON_ARROW_DOWN: u8 = 12;
const JOYSTICK_BUTTON_ARROW_LEFT: u8 = 13;
const JOYSTICK_BUTTON_ARROW_RIGHT: u8 = 14;
const JOYSTICK_AXIS_LEFT_X: u8 = 0;
const JOYSTICK_AXIS_LEFT_Y: u8 = 1;
const JOYSTICK_AXIS_RIGHT_X: u8 = 2;
const JOYSTICK_AXIS_RIGHT_Y: u8 = 3;
const JOYSTICK_AXIS_L2: u8 = 4;
const JOYSTICK_AXIS_R2: u8 = 5;


struct ControllerState {
    x: bool,
    circle: bool,
    square: bool,
    triangle: bool,
    select: bool,
    start: bool,
    l1: bool,
    r1: bool,
    l2: i16,
    r2: i16,
    yaw: i16,
    throttle: i16,
    pitch: i16,
    roll: i16
}

fn print_controller_state(state: &ControllerState) {
    println!("{{");
    println!("  X: {}", state.x);
    println!("  Circle: {}", state.circle);
    println!("  Square: {}", state.square);
    println!("  Triangle: {}", state.triangle);
    println!("  Select: {}", state.select);
    println!("  Start: {}", state.start);
    println!("  L1: {}", state.l1);
    println!("  R1: {}", state.r1);
    println!("  L2: {}", state.l2);
    println!("  R2: {}", state.r2);
    println!("  Yaw: {}", state.yaw);
    println!("  Throttle: {}", state.throttle);
    println!("  Pitch: {}", state.pitch);
    println!("  Roll: {}", state.roll);
    println!("}}");
}

fn update_controller_state(state: &mut ControllerState, event: &Event) {
    match event {
        Event::JoyAxisMotion { axis_idx, value, .. } => {
            match *axis_idx {
                JOYSTICK_AXIS_LEFT_X => {
                    state.yaw = *value;
                },
                JOYSTICK_AXIS_LEFT_Y => {
                    state.throttle = *value;
                },
                JOYSTICK_AXIS_RIGHT_X => {
                    state.roll = *value;
                },
                JOYSTICK_AXIS_RIGHT_Y => {
                    state.pitch = *value;
                },
                JOYSTICK_AXIS_L2 => {
                    state.l2 = *value;
                },
                JOYSTICK_AXIS_R2 => {
                    state.r2 = *value;
                },
                _ => {}
            }
        },
        Event::JoyButtonDown { button_idx, .. } => {
            match *button_idx {
                JOYSTICK_BUTTON_X => {
                    state.x = true;
                },
                JOYSTICK_BUTTON_CIRCLE => {
                    state.circle = true;
                },
                JOYSTICK_BUTTON_SQUARE => {
                    state.square = true;
                },
                JOYSTICK_BUTTON_TRIANGLE => {
                    state.triangle = true;
                },
                JOYSTICK_BUTTON_SELECT => {
                    state.select = true;
                },
                JOYSTICK_BUTTON_START => {
                    state.start = true;
                },
                JOYSTICK_BUTTON_L1 => {
                    state.l1 = true;
                },
                JOYSTICK_BUTTON_R1 => {
                    state.r1 = true;
                },
                _ => {}
            }
        },
        Event::JoyButtonUp { button_idx, .. } => {
            match *button_idx {
                JOYSTICK_BUTTON_X => {
                    state.x = false;
                },
                JOYSTICK_BUTTON_CIRCLE => {
                    state.circle = false;
                },
                JOYSTICK_BUTTON_SQUARE => {
                    state.square = false;
                },
                JOYSTICK_BUTTON_TRIANGLE => {
                    state.triangle = false;
                },
                JOYSTICK_BUTTON_SELECT => {
                    state.select = false;
                },
                JOYSTICK_BUTTON_START => {
                    state.start = false;
                },
                JOYSTICK_BUTTON_L1 => {
                    state.l1 = false;
                },
                JOYSTICK_BUTTON_R1 => {
                    state.r1 = false;
                },
                _ => {}
            }
        },
        _ => {}
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let joystick_context = sdl_context.joystick().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut joystick: Option<sdl2::joystick::Joystick> = None;
    let mut controller_state = ControllerState {
        x: false,
        circle: false,
        square: false,
        triangle: false,
        select: false,
        start: false,
        l1: false,
        r1: false,
        l2: -32768,
        r2: -32768,
        yaw: 0,
        throttle: 0,
        pitch: 0,
        roll: 0
    };

    println!("Initialized SDL2");

    'running: loop {
        for event in event_pump.poll_iter() {
            update_controller_state(&mut controller_state, &event);

            match event {
                Event::Quit {..} => break 'running,
                Event::JoyDeviceAdded { timestamp: _, which } => {
                    match joystick_context.open(which) {
                        Ok(j) => {
                            println!("Joystick attached: {}", j.name());
                            joystick = Some(j);
                        },
                        Err(e) => {
                            println!("Failed to open joystick: {}", e);
                        }
                    }
                },
                Event::JoyDeviceRemoved { which, .. } => {
                    println!("Joystick detached: {}", which);
                },
                Event::JoyButtonUp { button_idx, .. } => {
                    if button_idx == JOYSTICK_BUTTON_ARROW_UP {
                        print_controller_state(&controller_state);
                    }
                },
                _ => {}
            }
        }
    }
}

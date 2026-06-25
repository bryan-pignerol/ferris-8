use std::{
    cell::RefCell,
    rc::Rc,
    thread::sleep,
    time::{Duration, Instant},
};

mod cartridge;
use cartridge::Cartridge;
mod display;
use display::Display;
mod input;
use input::Gamepad;

// WINDOW
const WINDOW_WIDTH: usize = 128;
const WINDOW_HEIGHT: usize = 128;

// DELTA TIME
const LOW_LIMIT: f32 = 0.0167; // 60 FPS Max
const HIGH_LIMIT: f32 = 1.0; // 10 FPS Min

fn main() {
    println!("Ferris-8 has Started !");

    let mut last_time: Instant = Instant::now();

    let mut window: Display = display::Display::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let shared_buffer: Rc<RefCell<Vec<u32>>> = window.get_buffer();
    let gamepad: Rc<RefCell<Gamepad>> = Rc::new(RefCell::new(input::Gamepad::new()));

    let mut app: Cartridge = cartridge::Cartridge::new(
        "examples/test_app.lua",
        Rc::clone(&shared_buffer),
        Rc::clone(&gamepad),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    let _ = app.ready();

    while window.update() {
        // DELTA TIME
        let mut delta_time: f32 = last_time.elapsed().as_secs_f32();

        if delta_time < LOW_LIMIT {
            delta_time = LOW_LIMIT
        } else if delta_time > HIGH_LIMIT {
            delta_time = HIGH_LIMIT
        }

        last_time = Instant::now();

        // GET KEYBOARD INPUTS
        gamepad.borrow_mut().update(window.get_window());

        // EXECUTE LUA SCRIPT
        let _ = app.update(delta_time);

        // DRAW
        let _ = app.draw(delta_time);
    }

    println!("Ferris-8 is closed !");
}

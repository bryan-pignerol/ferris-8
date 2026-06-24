use std::{cell::RefCell, rc::Rc};

mod cartridge;
use cartridge::Cartridge;
mod display;
use display::Display;
mod input;
use input::Gamepad;

const WINDOW_WIDTH: usize = 128;
const WINDOW_HEIGHT: usize = 128;

fn main() {
    println!("Fantasy Console is Started !");

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
        // GET KEYBOARD INPUTS
        gamepad.borrow_mut().update(window.get_window());

        // EXECUTE LUA SCRIPT
        let _ = app.update();

        // DRAW
        let _ = app.draw();
    }

    println!("Fantasy Console is closed !");
}

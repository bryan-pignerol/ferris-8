mod display;
use std::{cell::RefCell, rc::Rc};

use display::Display;
mod cartridge;
use cartridge::Cartridge;

const WINDOW_WIDTH: usize = 128;
const WINDOW_HEIGHT: usize = 128;

fn main() {
    println!("Fantasy Console is Started !");

    let mut window: Display = display::Display::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let shared_buffer: Rc<RefCell<Vec<u32>>> = window.get_buffer();

    let mut app: Cartridge = cartridge::Cartridge::new(
        "examples/test_app.lua",
        shared_buffer,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    );

    let black_color = 0x000000;

    let _ = app.ready();

    while window.update() {
        window.clear(black_color);

        // GET KEYBOARD INPUTS

        // EXECUTE LUA SCRIPT
        let _ = app.update();

        // DRAW
        let _ = app.draw();
    }

    println!("Fantasy Console is closed !");
}

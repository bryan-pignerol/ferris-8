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

    let mut app: Cartridge = cartridge::Cartridge::new("examples/test_app.lua", shared_buffer);

    let black_color = 0x000000;

    let _ = app.ready();

    while window.update() {
        window.clear(black_color);

        // GET KEYBOARD INPUTS

        // EXECUTE LUA SCRIPT
        // let _ = app.update();

        // DRAW
        //let _ = app.draw();
        let red_color = 0xFF0000;
        window.draw_pixel(64, 64, red_color);
        window.draw_rect(32, 32, 96, 96, red_color);
        window.draw_line(32, 32, 96, 96, red_color);
    }

    println!("Fantasy Console is closed !");
}

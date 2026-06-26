extern crate bresenham;
use bresenham::Bresenham;

use minifb::{Key, Scale, Window, WindowOptions};
use std::{cell::RefCell, rc::Rc};

use crate::display::font::*;

/// The struct used to create the application window.
pub struct Display {
    window: Window,
    buffer: Rc<RefCell<Vec<u32>>>,
    width: usize,
    height: usize,
}

impl Display {
    // WINDOW
    /// Create the window.
    pub fn new(width: usize, height: usize) -> Self {
        let buffer: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![0; width * height]));

        let mut options: WindowOptions = WindowOptions::default();
        options.scale = Scale::X4;

        let mut window = Window::new("My Fantasy Console in Rust !!!", width, height, options)
            .unwrap_or_else(|e| panic!("{}", e));

        window.set_target_fps(60);

        Self {
            window,
            buffer,
            width,
            height,
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn get_buffer(&self) -> Rc<RefCell<Vec<u32>>> {
        Rc::clone(&self.buffer)
    }

    /// Clear the window.
    pub fn clear(&mut self, color: u32) {
        let mut buffer = self.buffer.borrow_mut();
        for pixel in buffer.iter_mut() {
            *pixel = color;
        }
    }

    /// Update the window.
    pub fn update(&mut self) -> bool {
        self.window
            .update_with_buffer(&self.buffer.borrow(), self.width, self.height)
            .unwrap();

        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    // DRAWING
    /// Draw a pixel at specific coordinates.
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            self.buffer.borrow_mut()[index] = color;
        }
    }

    /// Draw a line on the window using the Bresenham's line algorithm.
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        for (x, y) in Bresenham::new((x1 as isize, y1 as isize), (x2 as isize, y2 as isize)) {
            self.draw_pixel(x as usize, y as usize, color);
        }
    }

    /// Draw a rectangle on the window
    pub fn draw_rect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        let (min_x, max_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
        let (min_y, max_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

        for current_x in min_x..=max_x {
            self.draw_pixel(current_x, y1, color);
            self.draw_pixel(current_x, y2, color);
        }

        for current_y in min_y..=max_y {
            self.draw_pixel(x1, current_y, color);
            self.draw_pixel(x2, current_y, color);
        }
    }

    pub fn draw_char(&mut self, character: char, start_x: usize, start_y: usize, color: u32) {
        let bitmap: [u8; 5] = match character {
            '1' => NUMBER_1,
            '2' => NUMBER_2,
            '3' => NUMBER_3,
            '4' => NUMBER_4,
            '5' => NUMBER_5,
            '6' => NUMBER_6,
            '7' => NUMBER_7,
            '8' => NUMBER_8,
            '9' => NUMBER_9,
            '0' => NUMBER_0,
            _ => NUMBER_0,
        };

        for y in 0..CHAR_HEIGHT {
            let line = bitmap[y];

            for x in 0..CHAR_WIDTH {
                let bit = (line >> (3 - x)) & 1;

                if bit == 1 {
                    self.draw_pixel(start_x + x, start_y + y, color);
                }
            }
        }
    }

    pub fn draw_text(&mut self, text: &str, start_x: usize, start_y: usize, color: u32) {
        let mut x = start_x;
        let mut y = start_y;

        for character in text.chars() {
            if character == '\n' {
                x = start_x;
                y += CHAR_HEIGHT + 1;
                continue;
            }

            self.draw_char(character, x, y, color);
            x += CHAR_WIDTH + 1;
        }
    }
}

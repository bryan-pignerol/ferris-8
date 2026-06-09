extern crate bresenham;
use bresenham::Bresenham;

use minifb::{Key, Scale, Window, WindowOptions};

/// The struct used to create the application window.
pub struct Video {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Video {
    // VIDEO
    /// Create the window.
    pub fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![0; width * height];

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

    /// Clear the window.
    pub fn clear(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }

    /// Update the window.
    pub fn update(&mut self) -> bool {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();

        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    // DRAWING
    /// Draw a pixel at specific coordinates.
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;

            self.buffer[index] = color;
        }
    }

    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        for (x, y) in Bresenham::new((x1 as isize, y1 as isize), (x2 as isize, y2 as isize)) {
            self.draw_pixel(x as usize, y as usize, color);
        }
    }

    pub fn draw_rect(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u32) {
        let (min_x, max_x) = if x1 <= x2 {
            (x1, x2)
        } else {
            (x2, x1)
        };
        let (min_y, max_y) = if y1 <= y2 {
            (y1, y2)
        } else {
            (y2, y1)
        };

        for current_x in min_x..=max_x {
            self.draw_pixel(current_x, y1, color);
            self.draw_pixel(current_x, y2, color);
        }

        for current_y in min_y..=max_y {
            self.draw_pixel(x1, current_y, color);
            self.draw_pixel(x2, current_y, color);
        }
    }
}

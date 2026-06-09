mod display;
use display::Video;

const WINDOW_WIDTH: usize = 128;
const WINDOW_HEIGHT: usize = 128;

fn main() {
    println!("Fantasy Console is Started !");

    let mut video: Video = display::Video::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let black_color = 0x000000;

    while video.update() {
        video.clear(black_color);

        // GET KEYBOARD INPUTS

        // EXECUTE LUA SCRIPT

        // DRAW
        let red_color = 0xFF0000;
        video.draw_pixel(64, 64, red_color);
        video.draw_rect(32, 32, 96, 96, red_color);
        video.draw_line(32, 32, 96, 96, red_color);
    }

    println!("Fantasy Console is closed !");
}

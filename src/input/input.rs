use minifb::{Key, Window};

/// The struct dedicated to the input.
/// 6 differents buttons :
///     - (0) Up    : Up Arrow
///     - (1) Down  : Down Arrow
///     - (2) Left  : Left Arrow
///     - (3) Right : Right Arrow
///     - (4) A     : Key X
///     - (5) B     : Key C
pub struct Gamepad {
    buttons: [bool; 6],
}

impl Gamepad {
    pub fn new() -> Gamepad {
        let buttons: [bool; 6] = [false, false, false, false, false, false];

        Self { buttons }
    }

    pub fn update(&mut self, window: &Window) {
        self.buttons[0] = window.is_key_down(Key::Up);
        self.buttons[1] = window.is_key_down(Key::Down);
        self.buttons[2] = window.is_key_down(Key::Left);
        self.buttons[3] = window.is_key_down(Key::Right);
        self.buttons[4] = window.is_key_down(Key::X);
        self.buttons[5] = window.is_key_down(Key::C);
    }

    pub fn is_pressed(&self, id: usize) -> bool {
        if id < 6 { self.buttons[id] } else { false }
    }
}

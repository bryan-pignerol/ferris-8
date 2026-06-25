extern crate bresenham;
use bresenham::Bresenham;

use mlua::{Function, Lua, Result};
use std::{cell::RefCell, fs, rc::Rc};

use crate::input::Gamepad;

pub struct Cartridge {
    lua: Lua,
    shared_buffer: Rc<RefCell<Vec<u32>>>,
    gamepad: Rc<RefCell<Gamepad>>,
}

impl Cartridge {
    /// Create the cartridge and init the lua file.
    pub fn new(
        script_path: &str,
        shared_buffer: Rc<RefCell<Vec<u32>>>,
        gamepad: Rc<RefCell<Gamepad>>,
        width: usize,
        height: usize,
    ) -> Self {
        let lua = Lua::new();

        let buffer: Rc<RefCell<Vec<u32>>> = Rc::clone(&shared_buffer);

        let lua_code = fs::read_to_string(script_path).expect("ERROR : Failed to read the file");
        Self::bind_constants_api(&lua);
        Self::bind_draw_api(&lua, buffer, width, height);
        Self::bind_input_api(&lua, Rc::clone(&gamepad));

        lua.load(&lua_code)
            .exec()
            .expect("ERROR: Failed to execute Lua code");

        Self {
            lua: lua,
            shared_buffer: shared_buffer,
            gamepad: gamepad,
        }
    }

    pub fn bind_constants_api(lua: &Lua) {
        // COLORS
        lua.globals()
            .set("BLACK", 0x000000)
            .expect("ERROR : Cannot add BLACK constant");
        lua.globals()
            .set("DARK_BLUE", 0x1D2B53)
            .expect("ERROR : Cannot add DARK_BLUE constant");
        lua.globals()
            .set("DARK_PURPLE", 0x7E2553)
            .expect("ERROR : Cannot add DARK_PURPLE constant");
        lua.globals()
            .set("DARK_GREEN", 0x008751)
            .expect("ERROR : Cannot add DARK_GREEN constant");
        lua.globals()
            .set("BROWN", 0xAB5236)
            .expect("ERROR : Cannot add BROWN constant");
        lua.globals()
            .set("DARK_GRAY", 0x5F574F)
            .expect("ERROR : Cannot add DARK_GRAY constant");
        lua.globals()
            .set("LIGHT_GRAY", 0xC2C3C7)
            .expect("ERROR : Cannot add LIGHT_GRAY constant");
        lua.globals()
            .set("WHITE", 0xFFF1E8)
            .expect("ERROR : Cannot add WHITE constant");
        lua.globals()
            .set("RED", 0xFF004D)
            .expect("ERROR : Cannot add RED constant");
        lua.globals()
            .set("ORANGE", 0xFFA300)
            .expect("ERROR : Cannot add ORANGE constant");
        lua.globals()
            .set("YELLOW", 0xFFEC27)
            .expect("ERROR : Cannot add YELLOW constant");
        lua.globals()
            .set("GREEN", 0x00E436)
            .expect("ERROR : Cannot add GREEN constant");
        lua.globals()
            .set("BLUE", 0x29ADFF)
            .expect("ERROR : Cannot add BLUE constant");
        lua.globals()
            .set("INDIGO", 0x83769C)
            .expect("ERROR : Cannot add INDIGO constant");
        lua.globals()
            .set("PINK", 0xFF77A8)
            .expect("ERROR : Cannot add PINK constant");
        lua.globals()
            .set("PEACH", 0xFFCCAA)
            .expect("ERROR : Cannot add PEACH constant");

        // INPUT
        lua.globals()
            .set("UP", 0)
            .expect("ERROR : Cannot add UP constant");
        lua.globals()
            .set("DOWN", 1)
            .expect("ERROR : Cannot add DOWN constant");
        lua.globals()
            .set("LEFT", 2)
            .expect("ERROR : Cannot add LEFT constant");
        lua.globals()
            .set("RIGHT", 3)
            .expect("ERROR : Cannot add RIGHT constant");
        lua.globals()
            .set("A", 4)
            .expect("ERROR : Cannot add A constant");
        lua.globals()
            .set("B", 5)
            .expect("ERROR : Cannot add B constant");
    }

    pub fn bind_draw_api(lua: &Lua, buffer: Rc<RefCell<Vec<u32>>>, width: usize, height: usize) {
        // clr function
        let buffer_for_clr = Rc::clone(&buffer);
        let clr = lua
            .create_function(move |_, color: u32| {
                for pixel in buffer_for_clr.borrow_mut().iter_mut() {
                    *pixel = color;
                }

                Ok(())
            })
            .expect("ERROR : Cannot create clr function");

        lua.globals()
            .set("clr", clr)
            .expect("ERROR: Cannot add clr function");

        // pset function
        let buffer_for_pset = Rc::clone(&buffer);
        let pset = lua
            .create_function(move |_, (x, y, color): (usize, usize, u32)| {
                if x < width && y < height {
                    let index = y * width + x;

                    buffer_for_pset.borrow_mut()[index] = color;
                }

                Ok(())
            })
            .expect("ERROR : Cannot create pset function");

        lua.globals()
            .set("pset", pset)
            .expect("ERROR: Cannot add pset function");

        // line function
        let buffer_for_line = Rc::clone(&buffer);
        let line = lua
            .create_function(
                move |_, (x1, y1, x2, y2, color): (usize, usize, usize, usize, u32)| {
                    let mut b = buffer_for_line.borrow_mut();
                    for (x, y) in
                        Bresenham::new((x1 as isize, y1 as isize), (x2 as isize, y2 as isize))
                    {
                        let index = y * width as isize + x;
                        b[index as usize] = color;
                    }
                    Ok(())
                },
            )
            .expect("ERROR : Cannot create line function");

        lua.globals()
            .set("line", line)
            .expect("ERROR: Cannot add line function");

        // rect function
        let buffer_for_rect = Rc::clone(&buffer);
        let rect = lua
            .create_function(
                move |_, (x1, y1, x2, y2, color): (usize, usize, usize, usize, u32)| {
                    let mut b = buffer_for_rect.borrow_mut();
                    let (min_x, max_x) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
                    let (min_y, max_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };

                    for current_x in min_x..=max_x {
                        let index_1 = y1 * width + current_x;
                        let index_2 = y2 * width + current_x;

                        b[index_1] = color;
                        b[index_2] = color;
                    }

                    for current_y in min_y..=max_y {
                        let index_1 = current_y * width + x1;
                        let index_2 = current_y * width + x2;

                        b[index_1] = color;
                        b[index_2] = color;
                    }
                    Ok(())
                },
            )
            .expect("ERROR : Cannot create rect function");

        lua.globals()
            .set("rect", rect)
            .expect("ERROR: Cannot add rect function");
    }

    pub fn bind_input_api(lua: &Lua, gamepad: Rc<RefCell<Gamepad>>) {
        let buffer_for_btn = Rc::clone(&gamepad);
        let btn = lua
            .create_function(move |_, id: usize| {
                let g = buffer_for_btn.borrow();
                Ok(g.is_pressed(id))
            })
            .expect("ERROR : Cannot create btn function");

        lua.globals()
            .set("btn", btn)
            .expect("ERROR: Cannot add btn function");
    }

    pub fn ready(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let ready_function: Function = globals.get("Ready")?;

        let () = ready_function.call(())?;

        Ok(())
    }

    pub fn update(&mut self, dt: f32) -> Result<()> {
        let globals = self.lua.globals();
        let update_function: Function = globals.get("Update")?;

        let () = update_function.call(dt)?;

        Ok(())
    }

    pub fn draw(&mut self, dt: f32) -> Result<()> {
        let globals = self.lua.globals();
        let draw_function: Function = globals.get("Draw")?;

        let () = draw_function.call(dt)?;

        Ok(())
    }
}

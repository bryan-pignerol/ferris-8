extern crate bresenham;
use bresenham::Bresenham;

use mlua::{Function, Lua, Result};
use std::{
    cell::{RefCell},
    fs,
    rc::Rc,
};

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

    pub fn update(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let update_function: Function = globals.get("Update")?;

        let () = update_function.call(())?;

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let draw_function: Function = globals.get("Draw")?;

        let () = draw_function.call(())?;

        Ok(())
    }
}

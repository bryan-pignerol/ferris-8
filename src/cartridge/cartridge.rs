use mlua::{Function, Lua, Result};
use std::{cell::RefCell, fs, rc::Rc};

pub struct Cartridge {
    lua: Lua,
    shared_buffer: Rc<RefCell<Vec<u32>>>,
}

impl Cartridge {
    /// Create the cartridge and init the lua file.
    pub fn new(
        script_path: &str,
        shared_buffer: Rc<RefCell<Vec<u32>>>,
        width: usize,
        height: usize,
    ) -> Self {
        let lua = Lua::new();

        let buffer: Rc<RefCell<Vec<u32>>> = Rc::clone(&shared_buffer);

        let lua_code = fs::read_to_string(script_path).expect("ERROR : Failed to read the file");

        // TODO: Add into an API function
        let pset = lua
            .create_function(move |_, (x, y, color): (usize, usize, u32)| {
                if x < width && y < height {
                    let index = y * width + x;

                    buffer.borrow_mut()[index] = color;
                }

                Ok(())
            })
            .expect("ERROR : Cannot create pset function");

        lua.globals()
            .set("pset", pset)
            .expect("ERROR: Cannot add pset function");

        lua.load(&lua_code)
            .exec()
            .expect("ERROR: Failed to execute Lua code");

        Self {
            lua: lua,
            shared_buffer: shared_buffer,
        }
    }

    // TODO: pub fn bind_api()

    pub fn ready(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let ready_function: Function = globals.get("Ready")?;

        let () = ready_function.call(())?;

        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let ready_function: Function = globals.get("Update")?;

        let () = ready_function.call(())?;

        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let ready_function: Function = globals.get("Draw")?;

        let () = ready_function.call(())?;

        Ok(())
    }
}

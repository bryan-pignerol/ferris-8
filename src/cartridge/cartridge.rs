use mlua::{Function, Lua, Result};
use std::{cell::{RefCell}, fs, rc::Rc};

pub struct Cartridge {
    lua: Lua,
    buffer: Rc<RefCell<Vec<u32>>>
}

impl Cartridge {
    /// Create the cartridge and init the lua file.
    pub fn new(script_path: &str, buffer: Rc<RefCell<Vec<u32>>>) -> Self {
        let lua = Lua::new();
        let lua_code = fs::read_to_string(script_path).expect("ERROR : Failed to read the file");

        lua.load(&lua_code)
            .exec()
            .expect("ERROR: Failed to execute Lua code");

        Self {
            lua: lua,
            buffer: buffer
        }
    }

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

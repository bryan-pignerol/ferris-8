use mlua::{Function, Lua, Result};
use std::fs;

pub struct Cartridge {
    script_path: String,
    lua: Lua,
}

impl Cartridge {
    pub fn new(script_path: &str) -> Self {
        let lua = Lua::new();
        let lua_code = fs::read_to_string(script_path).expect("ERROR : Failed to read the file");

        lua.load(&lua_code)
            .exec()
            .expect("ERROR: Failed to execute Lua code");

        Self {
            script_path: script_path.to_string(),
            lua: lua,
        }
    }

    pub fn ready(&mut self) -> Result<()> {
        let globals = self.lua.globals();
        let ready_function: Function = globals.get("Ready")?;

        let () = ready_function.call(())?;

        Ok(())
    }

    pub fn update(&mut self) {}

    pub fn draw(&mut self) {}
}

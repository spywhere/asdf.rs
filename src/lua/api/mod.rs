mod plugin;
mod process;

use crate::plugin::Plugin;

pub fn load(plugin: &Plugin, lua: &mlua::Lua) -> Result<(), mlua::Error> {
  let globals = lua.globals();

  let api = lua.create_table()?;

  let process = lua.create_table()?;
  process::load(lua, &process)?;
  api.set("process", process)?;

  globals.set("api", api)?;

  let pl = lua.create_table()?;
  plugin::load(lua, &pl, plugin)?;
  globals.set("plugin", pl)?;

  Ok(())
}

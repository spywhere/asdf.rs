mod plugin;
mod shell;

use crate::plugin::Plugin;

pub fn load(plugin: &Plugin, lua: &mlua::Lua) -> Result<(), mlua::Error> {
  let globals = lua.globals();

  let private = lua.create_table()?;
  private.set("data_dir", plugin.data_dir.clone())?;
  let plugin_dir = plugin
    .plugin_dir
    .clone()
    .into_os_string()
    .into_string()
    .unwrap_or("".to_string());
  private.set("plugin_dir", plugin_dir)?;
  globals.set("__private", private)?;

  let api = lua.create_table()?;
  shell::load(lua, &api)?;
  globals.set("api", api)?;

  let pl = lua.create_table()?;
  plugin::load(lua, &pl)?;
  globals.set("plugin", pl)?;

  Ok(())
}

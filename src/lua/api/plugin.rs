use crate::lua::unwrap_expect;
use crate::util;

use crate::plugin::Plugin;

pub fn load(
  lua: &mlua::Lua,
  table: &mlua::Table,
  plugin: &Plugin,
) -> Result<(), mlua::Error> {
  table.set("file", file(lua, plugin)?)?;

  Ok(())
}

fn file<'a>(
  lua: &'a mlua::Lua,
  plugin: &'a Plugin,
) -> Result<mlua::Function<'a>, mlua::Error> {
  let plugin_dir = plugin.plugin_dir.clone();

  let func = move |_, options: mlua::Value| {
    let path = unwrap_expect!(options, mlua::Value::String)?;

    let entry = util::path::check_exists(
      plugin_dir.join(path.to_str().unwrap_or("")),
      false,
    )
    .map(|v| v.to_str().unwrap_or("").to_string());

    Ok(entry)
  };

  lua.create_function(func)
}

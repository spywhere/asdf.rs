use std::path::PathBuf;

use crate::lua::{unwrap_expect, TableExpectation};
use crate::util;

pub fn load(lua: &mlua::Lua, table: &mlua::Table) -> Result<(), mlua::Error> {
  table.set("file", lua.create_function(file)?)?;

  Ok(())
}

fn file(
  lua: &mlua::Lua,
  options: mlua::Value,
) -> Result<Option<String>, mlua::Error> {
  let path = unwrap_expect!(options, mlua::Value::String)?;
  let globals = lua.globals();
  let private = globals.expect_get("__private".to_string())?;
  let private = unwrap_expect!(private, mlua::Value::Table)?;

  let plugin_dir = private.expect_get("plugin_dir".to_string())?;
  let plugin_dir = unwrap_expect!(plugin_dir, mlua::Value::String)?;
  let plugin_dir = plugin_dir.to_str()?.to_string();
  let plugin_dir = PathBuf::from(plugin_dir);

  let entry = util::path::check_exists(
    plugin_dir.join(path.to_str().unwrap_or("")),
    false,
  )
  .map(|v| v.to_str().unwrap_or("").to_string());

  Ok(entry)
}

use super::ExecutionError;

mod shell;

pub fn load(lua: &mlua::Lua) -> Result<(), ExecutionError> {
  let api = lua.create_table()?;
  shell::load(lua, &api)?;
  lua.globals().set("api", api)?;

  Ok(())
}

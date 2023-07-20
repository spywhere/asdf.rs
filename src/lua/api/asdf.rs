use crate::lua::unwrap_expect;

pub fn load(
  lua: &mlua::Lua,
  table: &mlua::Table,
) -> Result<(), mlua::Error> {
  table.set("split_whitespace", split_whitespace(lua)?)?;

  Ok(())
}

fn split_whitespace<'a>(
  lua: &'a mlua::Lua,
) -> Result<mlua::Function<'a>, mlua::Error> {
  let func = move |l: &'a mlua::Lua, value: mlua::Value| {
    let value: mlua::String = unwrap_expect(l, value)?;
    let value = value.to_str().unwrap_or_default().to_string();
    let values = value.split_whitespace();
    let list = l.create_table()?;

    for v in values {
      list.push(v)?;
    }

    Ok(list)
  };

  lua.create_function(func)
}

use crate::lua::{
  try_expect, LengthRequirement, RuntimeError, TableExpectation,
};

use crate::api::shell;

pub fn load(lua: &mlua::Lua, table: &mlua::Table) -> Result<(), mlua::Error> {
  table.set("spawn", lua.create_function(spawn)?)?;

  Ok(())
}

fn spawn<'a>(
  lua: &'a mlua::Lua,
  options: mlua::Value,
) -> Result<(i8, Option<mlua::Table<'a>>), mlua::Error> {
  let options = try_expect!(options, mlua::Value::Table)?;

  let args = options.try_get("args".to_string())?;
  let args = try_expect!(args, mlua::Value::Table)?;

  let mut args = args.sequence_values::<String>();

  let Some(command) = args.next() else {
    return Err(mlua::Error::external(RuntimeError::LengthRequired(
      "args".to_string(),
      LengthRequirement::Minimum(1)
    )))
  };

  let command = command?;
  let args = args.filter_map(|v| v.ok());
  let args = args.collect::<Vec<String>>();

  let mut output = None;
  let mut code = 0;
  match shell::spawn(&command, args, None) {
    Ok(result) => {
      let out = lua.create_table()?;
      out.set("exit_code", result.exit_code)?;
      out.set("stdout", result.stdout)?;
      out.set("stderr", result.stderr)?;
      output = Some(out)
    }
    Err(error) => {
      code = match error {
        shell::ShellError::Timedout => 1,
        shell::ShellError::ProcessError => 2,
        shell::ShellError::ReadError => 3,
      };
    }
  };

  Ok((code, output))
}

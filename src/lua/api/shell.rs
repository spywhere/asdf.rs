use super::ExecutionError;
use crate::cli::stdout;

use crate::api::shell;

pub fn load(
  lua: &mlua::Lua,
  table: &mlua::Table,
) -> Result<(), ExecutionError> {
  table.set("spawn", lua.create_function(spawn)?)?;

  Ok(())
}

fn spawn<'a>(
  lua: &'a mlua::Lua,
  options: mlua::Table,
) -> Result<(mlua::Table<'a>, u8), mlua::Error> {

  let args: mlua::Table = options.get("args")?;
  let mut args = args.sequence_values::<String>();

  let Some(command) = args.next() else {
    return Err(mlua::Error::RuntimeError("Command is expected".to_string()));
  };
  let command = command?;
  let args = args.filter_map(|v| v.ok());
  let args = args.collect::<Vec<String>>();

  let output = lua.create_table()?;
  let mut code = 0;
  stdout!("{} {:?}", command, args);
  match shell::spawn(&command, args, None) {
    Ok(result) => {
      stdout!("building output");
      output.set("exit_code", result.exit_code)?;
      output.set("stdout", result.stdout)?;
      output.set("stderr", result.stderr)?;
      stdout!("finish output");
    }
    Err(error) => {
      stdout!("building error");
      code = match error {
        shell::ShellError::Timedout => 1,
        shell::ShellError::ProcessError => 2,
        shell::ShellError::ReadError => 3,
      };
    }
  };

  Ok((output, code))
}

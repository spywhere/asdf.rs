use crate::lua::{unwrap_expect, TableExpectation};

use crate::api::shell;

pub fn load(lua: &mlua::Lua, table: &mlua::Table) -> Result<(), mlua::Error> {
  table.set("spawn", spawn(lua)?)?;

  Ok(())
}

fn spawn<'a>(lua: &'a mlua::Lua) -> Result<mlua::Function<'a>, mlua::Error> {
  let func = move |l: &'a mlua::Lua,
                   options: mlua::Value|
        -> Result<(i8, Option<mlua::Table<'a>>), mlua::Error> {
    let options = unwrap_expect!(options, mlua::Value::Table)?;

    let command = options.expect_get("command".to_string())?;
    let command = unwrap_expect!(command, mlua::Value::String)?;
    let command = command.to_str()?.to_string();

    let opt_args = options.try_get("args".to_string())?;
    let mut args: Vec<String> = Vec::new();

    if let Some(opt_args) = opt_args {
      let opt_args = unwrap_expect!(opt_args, mlua::Value::Table)?;
      let opt_args = opt_args.sequence_values::<String>();
      let opt_args = opt_args.filter_map(|v| v.ok());
      args = opt_args.collect::<Vec<String>>();
    }

    let mut output = None;
    let mut code = 0;
    match shell::spawn(&command, args, None) {
      Ok(result) => {
        let out = l.create_table()?;
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
  };

  lua.create_function(func)
}

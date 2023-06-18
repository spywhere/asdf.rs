use crate::cli::Exit;

use crate::cmd::Context;

use crate::plugin;

use crate::lua::{self, PluginExecutable};

pub struct ListAllOptions {
  pub plugin: String,
  pub version: Option<String>,
}

pub fn list_all(
  context: Context,
  options: ListAllOptions,
) -> Result<(), Exit> {
  let name = options.plugin;
  let plugin = plugin::get(&context, name)?;

  if let Err(err) = plugin.execute(lua::EntryPoint::Main) {
    return Err(Exit {
      exit_code: 1,
      message: Some(match err {
        lua::ExecutionError::LoadingError => "Loading error".to_string(),
        lua::ExecutionError::InvalidSyntax(str) => str,
        lua::ExecutionError::RuntimeError(str) => str,
      }),
    });
  }

  Ok(())
}

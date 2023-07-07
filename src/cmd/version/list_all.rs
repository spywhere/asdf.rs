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

  plugin.execute(lua::EntryPoint::Main).map_err(|e| Exit {
    exit_code: 1,
    message: Some(e.to_string())
  })
}

use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

use crate::plugin;

pub struct ListOptions {
  pub urls: bool,
  pub refs: bool,
}

pub fn list(context: Context, _options: ListOptions) -> Result<(), Exit> {
  let plugins = plugin::plugins(&context)?;

  for plugin in plugins {
    stdout!("{}", plugin.name);
  }

  Ok(())
}

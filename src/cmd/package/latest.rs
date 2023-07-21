use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

use crate::plugin;

pub struct LatestOptions {
  pub name: Option<String>,
  pub prefix: Option<String>,
}

pub fn latest(context: Context, options: LatestOptions) -> Result<(), Exit> {
  let Some(name) = options.name else {
    return latest_all(context);
  };

  stdout!("List latest version for {}", name);

  Ok(())
}

fn latest_all(context: Context) -> Result<(), Exit> {
  stdout!("List latest version for all plugins");

  let plugins = plugin::plugins(&context)?;

  for plugin in plugins {
    stdout!("{}", plugin.name);
  }

  Ok(())
}

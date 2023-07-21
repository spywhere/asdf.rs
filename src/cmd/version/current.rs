use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct CurrentOptions {
  pub plugin: Option<String>,
}

pub fn current(context: Context, options: CurrentOptions) -> Result<(), Exit> {
  match options.plugin {
    Some(plugin) => current_for_plugin(context, plugin),
    None => current_for_all(context),
  }
}

fn current_for_plugin(_context: Context, name: String) -> Result<(), Exit> {
  stdout!("current version for {}", name);

  Ok(())
}

fn current_for_all(_context: Context) -> Result<(), Exit> {
  stdout!("current version for all plugins");

  Ok(())
}

use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct UpdateOptions {
  pub name: Option<String>,
  pub git_ref: Option<String>,
}

pub fn update(context: Context, options: UpdateOptions) -> Result<(), Exit> {
  let Some(name) = options.name else {
    return update_all(context);
  };

  stdout!("Update a plugin {}", name);

  Ok(())
}

pub fn update_all(_context: Context) -> Result<(), Exit> {
  stdout!("Update all plugins");

  Ok(())
}

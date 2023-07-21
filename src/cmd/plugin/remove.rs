use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct RemoveOptions {
  pub name: String,
}

pub fn remove(_context: Context, options: RemoveOptions) -> Result<(), Exit> {
  stdout!("Remove a plugin {}", options.name);

  Ok(())
}

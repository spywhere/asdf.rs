use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct ShellOptions {
  pub plugin: String,
  pub version: String,
}

pub fn shell(_context: Context, options: ShellOptions) -> Result<(), Exit> {
  stdout!("set shell {} to {}", options.plugin, options.version);

  Ok(())
}

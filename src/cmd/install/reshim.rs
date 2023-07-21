use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct ReshimOptions {
  pub plugin: String,
  pub version: String,
}

pub fn reshim(_context: Context, options: ReshimOptions) -> Result<(), Exit> {
  stdout!("reshim {} {}", options.plugin, options.version);

  Ok(())
}

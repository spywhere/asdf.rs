use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct LocalOptions {
  pub plugin: String,
  pub version: String,
}

pub fn local(_context: Context, options: LocalOptions) -> Result<(), Exit> {
  stdout!("set local {} to {}", options.plugin, options.version);

  Ok(())
}

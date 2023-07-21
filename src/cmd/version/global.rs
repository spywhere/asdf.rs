use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct GlobalOptions {
  pub plugin: String,
  pub version: String,
}

pub fn global(_context: Context, options: GlobalOptions) -> Result<(), Exit> {
  stdout!("set global {} to {}", options.plugin, options.version);

  Ok(())
}

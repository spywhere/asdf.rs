use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct UninstallOptions {
  pub name: String,
  pub version: String,
}

pub fn uninstall(
  _context: Context,
  options: UninstallOptions,
) -> Result<(), Exit> {
  stdout!("Remove {} {}", options.name, options.version);

  Ok(())
}

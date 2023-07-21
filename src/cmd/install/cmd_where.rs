use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct WhereOptions {
  pub plugin: String,
  pub version: Option<String>,
}

pub fn cmd_where(
  _context: Context,
  options: WhereOptions,
) -> Result<(), Exit> {
  stdout!("where for {}", options.plugin);

  Ok(())
}

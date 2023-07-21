use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub fn list_all(_context: Context) -> Result<(), Exit> {
  stdout!("fetch from repo");
  Ok(())
}

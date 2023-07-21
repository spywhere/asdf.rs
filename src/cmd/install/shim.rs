use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct ShimOptions {
  pub command: String,
}

pub fn shim(_context: Context, options: ShimOptions) -> Result<(), Exit> {
  stdout!("Shim for {}", options.command);

  Ok(())
}

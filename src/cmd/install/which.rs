use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct WhichOptions {
  pub command: String,
}

pub fn which(_context: Context, options: WhichOptions) -> Result<(), Exit> {
  stdout!("Which for {}", options.command);

  Ok(())
}

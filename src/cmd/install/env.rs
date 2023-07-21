use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct EnvOptions {
  pub command: String,
  pub util: String,
}

pub fn env(_context: Context, options: EnvOptions) -> Result<(), Exit> {
  stdout!("Env for {}", options.command);

  Ok(())
}

use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct ExecOptions {
  pub command: String,
  pub args: Vec<String>,
}

pub fn exec(_context: Context, options: ExecOptions) -> Result<(), Exit> {
  stdout!("Exec for {}", options.command);

  Ok(())
}

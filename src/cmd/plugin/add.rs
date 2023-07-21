use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct AddOptions {
  pub name: String,
  pub git_url: Option<String>,
}

pub fn add(_context: Context, options: AddOptions) -> Result<(), Exit> {
  stdout!("Add a new plugin {}", options.name);

  Ok(())
}

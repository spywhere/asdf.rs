use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct InstallOptions {
  pub name: Option<String>,
  pub version: Option<String>,
}

pub fn install(context: Context, options: InstallOptions) -> Result<(), Exit> {
  let Some(name) = options.name else {
    return install_all_from_manifest(context);
  };

  let Some(version) = options.version else {
    return install_from_manifest(context, name);
  };

  stdout!("Install {} {}", name, version);

  Ok(())
}

fn install_from_manifest(_context: Context, name: String) -> Result<(), Exit> {
  stdout!("Install {} from version file", name);

  Ok(())
}

fn install_all_from_manifest(_context: Context) -> Result<(), Exit> {
  stdout!("Install from version file");

  Ok(())
}

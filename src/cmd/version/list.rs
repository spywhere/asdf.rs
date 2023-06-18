use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

use crate::plugin::{self as plugin, PackageError, PackageVersion};

pub struct ListOptions {
  pub plugin: Option<String>,
  pub version: Option<String>,
}

pub fn list(context: Context, options: ListOptions) -> Result<(), Exit> {
  match options.plugin {
    Some(plugin) => list_plugin(context, plugin, options.version),
    None => list_all_plugin(context),
  }
}

fn list_all_plugin(context: Context) -> Result<(), Exit> {
  stdout!("list in {}", context.data_dir);
  Ok(())
}

fn list_plugin(
  context: Context,
  plugin: String,
  version: Option<String>,
) -> Result<(), Exit> {
  let name = plugin.clone();
  let plugin = plugin::get(&context, plugin)?;
  let versions = plugin.versions(&version)?;

  if versions.is_empty() {
    return match version {
      Some(version) => Err(Exit {
        exit_code: 1,
        message: Some(format!(
          "No compatible version installed ({} {})",
          name, version
        )),
      }),
      None => Err(PackageError::NoInstallation.into()),
    }
  }

  for version in versions {
    stdout!("  {}", version);
  }

  Ok(())
}

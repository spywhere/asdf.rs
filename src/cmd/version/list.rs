use std::fs;

use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

use crate::util;

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
  let install_dir = util::path::get(
    context,
    util::path::CommonPath::Install {
      plugin: plugin.clone(),
      version: version.clone(),
    },
  );

  let Some(install_dir) = install_dir else {
    return Err(Exit {
      exit_code: 0,
      message: Some("No version installed".to_string()),
    });
  };

  let Ok(entries) = fs::read_dir(install_dir) else {
    return Err(Exit {
      exit_code: 1,
      message: Some(format!("Cannot fetch installed version for {}", plugin))
    })
  };

  let mut versions: Vec<String> = entries
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_dir())
    .map(|e| e.file_name().into_string().unwrap_or_default())
    .filter(|v| {
      version
        .as_ref()
        .map_or(true, |ver| v.starts_with(ver.as_str()))
    })
    .collect();

  if let (Some(version), true) = (version, versions.is_empty()) {
    return Err(Exit {
      exit_code: 1,
      message: Some(format!(
        "No compatible version installed ({} {})",
        plugin, version
      )),
    });
  }

  versions.sort();

  for version in versions {
    stdout!("  {}", version);
  }

  Ok(())
}

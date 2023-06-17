use std::fs;
use std::path::Path;

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
    Some(plugin) => list_plugin(plugin, options.version, context.data_dir),
    None => {
      stdout!("list in {}", context.data_dir);
      Ok(())
    }
  }
}

fn list_plugin(plugin: String, version: Option<String>, data_dir: String) -> Result<(), Exit> {
  let install_dir = Path::new(&data_dir).join("installs").join(plugin.clone());
  let install_dir = util::expand_path(install_dir);

  let is_dir = install_dir.is_dir();

  if !matches!(install_dir.try_exists(), Ok(true)) || !is_dir {
    return Err(Exit {
      exit_code: 0,
      message: Some("No version installed".to_string()),
    });
  }

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

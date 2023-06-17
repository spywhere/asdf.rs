use std::path::{Path, PathBuf};

use crate::cmd::Context;

pub fn expand<P: AsRef<Path>>(path: P) -> PathBuf {
  let path = path.as_ref();

  if !path.starts_with("~") {
    return path.to_path_buf();
  }

  let mut home =
    dirs::home_dir().unwrap_or_else(|| Path::new("~").to_path_buf());
  if path == Path::new("~") {
    return home;
  }

  let homeless_path = path.strip_prefix("~").unwrap_or(path);

  if home.as_path() == Path::new("/") {
    return homeless_path.to_path_buf();
  }

  home.push(homeless_path);
  home
}

#[derive(Clone)]
pub enum CommonPath {
  Install { plugin: String, version: Option<String>, },
  Plugin(String),
  PluginBinary { plugin: String, binary: String },
}

pub fn get(context: &Context, path: CommonPath) -> Option<PathBuf> {
  let dir_path = Path::new(&context.data_dir);
  let dir_path = expand(dir_path);

  let dir_path = match path.clone() {
    CommonPath::Install { plugin, version } => {
      let mut path = dir_path.join("installs").join(plugin);
      if let Some(version) = version {
        path = path.join(version);
      }
      path
    },
    CommonPath::Plugin(plugin) => {
      dir_path.join("plugins").join(plugin)
    },
    CommonPath::PluginBinary { plugin, binary } => {
      dir_path.join("plugins").join(plugin).join("bin").join(binary)
    },
  };

  let is_dir = match path {
    CommonPath::PluginBinary { .. } => dir_path.is_file(),
    _ => dir_path.is_dir(),
  };

  if !matches!(dir_path.try_exists(), Ok(true)) || !is_dir {
    return None;
  }

  Some(dir_path)
}

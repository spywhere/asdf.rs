use std::path::{Path, PathBuf};

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
  Install {
    plugin: String,
    version: Option<String>,
  },
  Plugin(Option<String>),
}

pub fn check_exists(path: PathBuf, expect_dir: bool) -> Option<PathBuf> {
  let is_dir = match expect_dir {
    true => path.is_dir(),
    false => path.is_file(),
  };

  if !matches!(path.try_exists(), Ok(true)) || !is_dir {
    return None;
  }

  Some(path)
}

pub fn get(data_dir: &String, path: CommonPath) -> Option<PathBuf> {
  let dir_path = Path::new(&data_dir);
  let dir_path = expand(dir_path);

  let dir_path = match path {
    CommonPath::Install { plugin, version } => {
      let mut path = dir_path.join("installs").join(plugin);
      if let Some(version) = version {
        path = path.join(version);
      }
      path
    }
    CommonPath::Plugin(plugin) => {
      let mut path = dir_path.join("plugins");
      if let Some(plugin) = plugin {
        path = path.join(plugin)
      }
      path
    },
  };

  check_exists(dir_path, true)
}

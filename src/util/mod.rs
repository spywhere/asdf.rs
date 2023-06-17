use std::path::{Path, PathBuf};

pub fn expand_path<P: AsRef<Path>>(path: P) -> PathBuf {
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

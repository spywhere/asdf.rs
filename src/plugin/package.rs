use std::fs;
use std::path::PathBuf;

use crate::util;

pub enum PackageError {
  NoInstallation,
  FetchError(String),
}

pub struct Package {
  pub(super) data_dir: String,
  pub plugin: String,
  pub(super) plugin_dir: PathBuf,
  pub version: String,
  pub(super) version_dir: PathBuf,
}

pub trait PackageVersion {
  fn versions(
    &self,
    prefix: &Option<String>,
  ) -> Result<Vec<String>, PackageError>;
  // fn version(&self, version: String) -> Package;
}

impl PackageVersion for super::Plugin {
  fn versions(
    &self,
    prefix: &Option<String>,
  ) -> Result<Vec<String>, PackageError> {
    let name = &self.name;

    let install_dir = util::path::get(
      &self.data_dir,
      util::path::CommonPath::Install {
        plugin: name.to_string(),
        version: None,
      },
    );

    let Some(install_dir) = install_dir else {
      return Err(PackageError::NoInstallation);
    };

    let Ok(entries) = fs::read_dir(install_dir) else {
      return Err(PackageError::FetchError(name.to_string()));
    };

    let mut versions: Vec<String> = entries
      .filter_map(|e| e.ok())
      .filter(|e| e.path().is_dir())
      .map(|e| e.file_name().into_string().unwrap_or_default())
      .filter(|v| {
        prefix
          .as_ref()
          .map_or(true, |ver| v.starts_with(ver.as_str()))
      })
      .collect();

    versions.sort();

    Ok(versions)
  }
}

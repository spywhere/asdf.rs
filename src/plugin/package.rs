use std::fs;
use std::path::PathBuf;

use crate::util;

pub enum PackageError {
  NoInstallation,
  FetchError(String),
  NotFound { plugin: String, version: String },
}

pub struct Package {
  pub plugin: super::Plugin,
  pub version: String,
  pub(super) version_dir: PathBuf,
}

pub trait PackageVersion {
  fn versions(
    &self,
    prefix: &Option<String>,
  ) -> Result<Vec<Package>, PackageError>;
  fn version(&self, version: String) -> Result<Package, PackageError>;
}

impl PackageVersion for super::Plugin {
  fn versions(
    &self,
    prefix: &Option<String>,
  ) -> Result<Vec<Package>, PackageError> {
    let name = &self.name;

    let install_dir = util::path::get(
      &self.data_dir,
      util::path::CommonPath::Install {
        plugin: name.to_string(),
        version: None,
      },
    );

    let install_dir = install_dir.ok_or(PackageError::NoInstallation)?;

    let entries = fs::read_dir(install_dir.clone())
      .map_err(|_| PackageError::FetchError(name.to_string()))?;

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

    let packages: Vec<Package> = versions
      .iter()
      .map(|v| self.version(v.to_string()))
      .filter_map(|p| p.ok())
      .collect();

    Ok(packages)
  }

  fn version(&self, version: String) -> Result<Package, PackageError> {
    let name = &self.name;

    let install_dir = util::path::get(
      &self.data_dir,
      util::path::CommonPath::Install {
        plugin: name.to_string(),
        version: Some(version.clone()),
      },
    );

    let install_dir = install_dir.ok_or(PackageError::NotFound {
      plugin: name.to_string(),
      version: version.clone(),
    })?;

    Ok(Package {
      plugin: self.clone(),
      version: version.to_string(),
      version_dir: install_dir,
    })
  }
}

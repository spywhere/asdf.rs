use crate::cli::options::{OptionalPluginArgs, PackageArgs};

pub type PackageLatestOptions = OptionalPluginArgs;

use crate::cmd::package::LatestOptions;

impl From<PackageLatestOptions> for LatestOptions {
  fn from(args: PackageLatestOptions) -> Self {
    Self {
      name: args.name,
      prefix: args.version,
    }
  }
}

pub type PackageInstallOptions = OptionalPluginArgs;

use crate::cmd::package::InstallOptions;

impl From<PackageInstallOptions> for InstallOptions {
  fn from(args: PackageInstallOptions) -> Self {
    Self {
      name: args.name,
      version: args.version,
    }
  }
}

pub type PackageUninstallOptions = PackageArgs;

use crate::cmd::package::UninstallOptions;

impl From<PackageUninstallOptions> for UninstallOptions {
  fn from(args: PackageUninstallOptions) -> Self {
    Self {
      name: args.name,
      version: args.version,
    }
  }
}

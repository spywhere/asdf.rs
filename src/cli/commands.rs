use clap::Subcommand;

use crate::cli::options::*;

#[derive(Subcommand, Clone)]
pub enum Commands {
  /// Plugin management
  #[command()]
  Plugin(PluginOptions),

  /// Show latest stable version
  Latest(PackageLatestOptions),
  /// Install a specific version of a package
  #[command()]
  Install(PackageInstallOptions),
  /// Remove a specific version of a package
  #[command()]
  Uninstall(PackageUninstallOptions),

  /// Display current version set or being used for all packages
  #[command()]
  Current(VersionCurrentOptions),
  /// Display installed versions of a package
  #[command(
    subcommand_negates_reqs = true,
    subcommand_precedence_over_arg = true
  )]
  List(VersionListOptions),
  /// Set the package global version
  #[command()]
  Global(VersionGlobalOptions),
  /// Set the package local version
  #[command()]
  Local(VersionLocalOptions),
  /// Set the package version in the current shell session
  #[command()]
  Shell(VersionShellOptions),

  /// Display install path for a package
  #[command()]
  Where(InstallWhereOptions),
  /// Display the path to an executable
  #[command()]
  Which(InstallWhichOptions),
  /// Executes the command shim for current version
  #[command()]
  Exec(InstallExecOptions),
  /// Runs util inside the environment used for command shim execution
  #[command()]
  Env(InstallEnvOptions),
  /// Recreate shims for version of a package
  #[command()]
  Reshim(InstallReshimOptions),
  /// List plugins and versions that provide a command
  #[command()]
  ShimVersions(InstallShimVersionsOptions),
}

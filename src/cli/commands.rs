use clap::Subcommand;

use crate::cli::options::*;

#[derive(Subcommand, Clone)]
pub enum Commands {
  // Plugin Management //

  /// Plugin management
  #[command()]
  Plugin(PluginOptions),

  // Package Management //

  /// Show latest stable version
  #[command()]
  Latest(LatestCommandOptions),
  /// Install a specific version of a package
  #[command()]
  Install(InstallOptions),
  /// Remove a specific version of a package
  #[command()]
  Uninstall(UninstallOptions),

  // Version Management //

  /// Display current version set or being used for all packages
  #[command()]
  Current(CurrentOptions),
  /// Display installed versions of a package
  #[command(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
  List(ListCommandOptions),
  /// Set the package global version
  #[command()]
  Global(GlobalOptions),
  /// Set the package local version
  #[command()]
  Local(LocalOptions),
  /// Set the package version in the current shell session
  #[command()]
  Shell(ShellOptions),

  // Installation Management //

  /// Display install path for a package
  #[command()]
  Where(WhereOptions),
  /// Display the path to an executable
  #[command()]
  Which(WhichOptions),
  /// Executes the command shim for current version
  #[command()]
  Exec(ExecOptions),
  /// Runs util inside the environment used for command shim execution
  #[command()]
  Env(EnvOptions),
  /// Recreate shims for version of a package
  #[command()]
  Reshim(ReshimOptions),
  /// List plugins and versions that provide a command
  #[command()]
  ShimVersions(ShimVersionsOptions),
}

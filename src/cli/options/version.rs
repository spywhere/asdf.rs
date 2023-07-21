use crate::cli::options::{OptionalPluginArgs, PackageArgs, PluginArgs};
use clap::{Args, Subcommand};

#[derive(Args, Clone)]
pub struct VersionCurrentOptions {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: Option<String>,
}

use crate::cmd::version::CurrentOptions;

impl From<VersionCurrentOptions> for CurrentOptions {
  fn from(args: VersionCurrentOptions) -> Self {
    Self { plugin: args.name }
  }
}

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct VersionListOptions {
  #[command(subcommand)]
  pub command: Option<ListCommands>,

  #[command(flatten)]
  pub list: OptionalPluginArgs,
}

#[derive(Subcommand, Clone)]
pub enum ListCommands {
  /// List all versions of a package
  #[command()]
  All(PluginArgs),
}

use crate::cmd::version::{ListAllOptions, ListOptions};

impl From<PluginArgs> for ListAllOptions {
  fn from(args: PluginArgs) -> Self {
    Self {
      plugin: args.name,
      version: args.version,
    }
  }
}

impl From<OptionalPluginArgs> for ListOptions {
  fn from(args: OptionalPluginArgs) -> Self {
    Self {
      plugin: args.name,
      version: args.version,
    }
  }
}

pub type VersionGlobalOptions = PackageArgs;
pub type VersionLocalOptions = PackageArgs;
pub type VersionShellOptions = PackageArgs;

use crate::cmd::version::{GlobalOptions, LocalOptions, ShellOptions};

impl From<VersionGlobalOptions> for GlobalOptions {
  fn from(args: VersionGlobalOptions) -> Self {
    Self {
      plugin: args.name,
      version: args.version,
    }
  }
}

impl From<VersionLocalOptions> for LocalOptions {
  fn from(args: VersionLocalOptions) -> Self {
    Self {
      plugin: args.name,
      version: args.version,
    }
  }
}

impl From<VersionShellOptions> for ShellOptions {
  fn from(args: VersionShellOptions) -> Self {
    Self {
      plugin: args.name,
      version: args.version,
    }
  }
}

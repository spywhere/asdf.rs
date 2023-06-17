use crate::cli::options::{OptionalPluginArgs, PackageArgs, PluginArgs};
use clap::{Args, Subcommand};

#[derive(Args, Clone)]
pub struct CurrentOptions {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: Option<String>,
}

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ListCommandOptions {
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

use crate::cmd::{ListAllOptions, ListOptions};

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

pub type GlobalOptions = PackageArgs;
pub type LocalOptions = PackageArgs;
pub type ShellOptions = PackageArgs;

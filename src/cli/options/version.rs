use clap::{Args, Subcommand};
use crate::cli::options::{PluginArgs, PackageArgs};

#[derive(Args)]
pub struct CurrentOptions {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: Option<String>,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ListCommandOptions {
  #[command(subcommand)]
  pub command: Option<ListCommands>,

  #[command(flatten)]
  pub list: PluginArgs,
}

#[derive(Subcommand)]
pub enum ListCommands {
  /// List all versions of a package
  #[command()]
  All(PluginArgs),
}

pub type GlobalOptions = PackageArgs;
pub type LocalOptions = PackageArgs;
pub type ShellOptions = PackageArgs;

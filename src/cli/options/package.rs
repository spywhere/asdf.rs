use clap::{Args, Subcommand};

use crate::cli::options::{PluginArgs, PackageArgs};

pub type LatestOptions = PluginArgs;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct LatestCommandOptions {
  #[command(subcommand)]
  pub command: Option<LatestCommands>,

  #[command(flatten)]
  pub latest: LatestOptions,
}

#[derive(Subcommand, Clone)]
pub enum LatestCommands {
  /// Show latest stable version of all installed packages
  #[command(name = "--all")]
  All,
}

#[derive(Args, Clone)]
pub struct InstallOptions {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: Option<String>,

  /// Package version
  #[arg(value_name = "version")]
  pub version: Option<String>,
}

pub type UninstallOptions = PackageArgs;

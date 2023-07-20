use clap::{Args, Subcommand};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PluginOptions {
  #[command(subcommand)]
  pub command: PluginCommands,
}

#[derive(Subcommand, Clone)]
pub enum PluginCommands {
  /// Add a plugin from the plugin repository or from the given repository URL
  #[command()]
  Add(PluginAddOptions),
  /// List installed plugins
  #[command()]
  List(PluginListCommandOptions),
  /// Remove plugin and package versions
  #[command()]
  Remove(PluginRemoveOptions),
  /// Update a plugin to latest commit
  #[command()]
  Update(PluginUpdateCommandOptions),
}

#[derive(Args, Clone)]
pub struct PluginAddOptions {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: String,

  /// Git repository URL
  #[arg(value_name = "git-url")]
  pub git_url: Option<String>,
}

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PluginListCommandOptions {
  #[command(subcommand)]
  pub command: Option<PluginListCommands>,

  /// Show plugin's git repository URL
  #[arg(long)]
  pub urls: bool,

  /// Show plugin's git ref
  #[arg(long)]
  pub refs: bool,
}

use crate::cmd::plugin::ListOptions;

impl From<PluginListCommandOptions> for ListOptions {
  fn from(args: PluginListCommandOptions) -> Self {
    Self {
      urls: args.urls,
      refs: args.refs,
    }
  }
}

#[derive(Subcommand, Clone)]
pub enum PluginListCommands {
  /// List all available plugins on the plugin repository
  #[command()]
  All,
}

#[derive(Args, Clone)]
pub struct PluginRemoveOptions {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: String,
}

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PluginUpdateCommandOptions {
  #[command(subcommand)]
  pub command: Option<PluginUpdateCommands>,

  /// Plugin name
  #[arg(value_name = "name")]
  pub name: String,

  /// Git repository URL
  #[arg(value_name = "git-ref")]
  pub git_ref: Option<String>,
}

#[derive(Subcommand, Clone)]
pub enum PluginUpdateCommands {
  /// List all available plugins on the plugin repository
  #[command(name = "--all")]
  All,
}

use clap::Args;

use crate::cli::options::{PluginArgs, PackageArgs};

pub type WhereOptions = PluginArgs;

#[derive(Args)]
pub struct WhichOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,
}

#[derive(Args)]
pub struct ExecOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,

  /// Command arguments
  #[arg(value_name = "args", trailing_var_arg = true)]
  pub args: Vec<String>,
}

#[derive(Args)]
pub struct EnvOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,

  /// Util name
  #[arg(value_name = "util", default_value = "env")]
  pub util: String,
}

pub type ReshimOptions = PackageArgs;

#[derive(Args)]
pub struct ShimVersionsOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,
}

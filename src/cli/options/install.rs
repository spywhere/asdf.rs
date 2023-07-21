use clap::Args;

use crate::cli::options::{PackageArgs, PluginArgs};

pub type InstallWhereOptions = PluginArgs;

use crate::cmd::install::WhereOptions;

impl From<InstallWhereOptions> for WhereOptions {
  fn from(args: InstallWhereOptions) -> Self {
    Self {
      plugin: args.name,
      version: args.version,
    }
  }
}

#[derive(Args, Clone)]
pub struct InstallWhichOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,
}

use crate::cmd::install::WhichOptions;

impl From<InstallWhichOptions> for WhichOptions {
  fn from(args: InstallWhichOptions) -> Self {
    Self {
      command: args.command,
    }
  }
}

#[derive(Args, Clone)]
pub struct InstallExecOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,

  /// Command arguments
  #[arg(value_name = "args", trailing_var_arg = true)]
  pub args: Vec<String>,
}

use crate::cmd::install::ExecOptions;

impl From<InstallExecOptions> for ExecOptions {
  fn from(args: InstallExecOptions) -> Self {
    Self {
      command: args.command,
      args: args.args,
    }
  }
}

#[derive(Args, Clone)]
pub struct InstallEnvOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,

  /// Util name
  #[arg(value_name = "util", default_value = "env")]
  pub util: String,
}

use crate::cmd::install::EnvOptions;

impl From<InstallEnvOptions> for EnvOptions {
  fn from(args: InstallEnvOptions) -> Self {
    Self {
      command: args.command,
      util: args.util,
    }
  }
}

pub type InstallReshimOptions = PackageArgs;

use crate::cmd::install::ReshimOptions;

impl From<InstallReshimOptions> for ReshimOptions {
  fn from(args: InstallReshimOptions) -> Self {
    Self {
      plugin: args.name,
      version: args.version,
    }
  }
}

#[derive(Args, Clone)]
pub struct InstallShimVersionsOptions {
  /// Command name
  #[arg(value_name = "command")]
  pub command: String,
}

use crate::cmd::install::ShimOptions;

impl From<InstallShimVersionsOptions> for ShimOptions {
  fn from(args: InstallShimVersionsOptions) -> Self {
    Self {
      command: args.command,
    }
  }
}

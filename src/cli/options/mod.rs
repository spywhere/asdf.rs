use clap::Args;

#[derive(Args, Clone)]
pub struct PluginArgs {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: String,

  /// Package version
  #[arg(value_name = "version")]
  pub version: Option<String>,
}

#[derive(Args, Clone)]
pub struct PackageArgs {
  /// Plugin name
  #[arg(value_name = "name")]
  pub name: String,

  /// Package version
  #[arg(value_name = "version")]
  pub version: String,
}

mod env;
pub use env::*;

mod plugin;
pub use plugin::*;

mod package;
pub use package::*;

mod version;
pub use version::*;

mod install;
pub use install::*;

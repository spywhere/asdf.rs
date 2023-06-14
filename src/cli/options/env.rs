use std::convert::From;
use crate::cmd::Context;

use clap::Args;

#[derive(Args, Clone)]
pub struct Envs {
  /// The path to the asdf data directory
  #[arg(hide_env_values = true, long = "data-dir", env = "ASDF_DATA_DIR", default_value = "~/.asdf")]
  pub data_dir: String,
}

impl From<Envs> for Context {
    fn from(envs: Envs) -> Self {
      Self {
        data_dir: envs.data_dir
      }
    }
}

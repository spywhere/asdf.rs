use clap::Args;

#[derive(Args)]
pub struct Envs {
  /// The path to the asdf data directory
  #[arg(hide_env_values = true, long = "data-dir", env = "ASDF_DATA_DIR", default_value = "~/.asdf")]
  pub data_dir: String,
}


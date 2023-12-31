pub use clap::Parser;

pub mod options;

mod commands;

pub use commands::Commands;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,

  #[command(flatten, next_help_heading = "Environment Variables")]
  pub envs: options::Envs,
}

pub struct Exit {
  pub exit_code: i32,
  pub message: Option<String>,
}

use crate::plugin::PluginError;
impl From<PluginError> for Exit {
  fn from(value: PluginError) -> Self {
    match value {
      PluginError::NoPlugin => Exit {
        exit_code: 1,
        message: Some("No plugin installed".to_string()),
      },
      PluginError::FetchError => Exit {
        exit_code: 1,
        message: Some("Cannot fetch plugins".to_string()),
      },
      PluginError::NotFound(name) => Exit {
        exit_code: 1,
        message: Some(format!("No such plugin: {}", name)),
      },
      PluginError::Corrupted { name, binary } => Exit {
        exit_code: 1,
        message: Some(format!(
          "Plugin {} is corrupted ({} expected)",
          name, binary
        )),
      },
    }
  }
}

use crate::plugin::PackageError;
impl From<PackageError> for Exit {
  fn from(value: PackageError) -> Self {
    match value {
      PackageError::NoInstallation => Exit {
        exit_code: 0,
        message: Some("No version installed".to_string()),
      },
      PackageError::FetchError(name) => Exit {
        exit_code: 1,
        message: Some(format!("Cannot fetch installed version for {}", name)),
      },
      PackageError::NotFound { plugin, version } => Exit {
        exit_code: 1,
        message: Some(format!(
          "Package version {} of {} plugin cannot be found",
          version, plugin
        )),
      },
    }
  }
}

#[macro_export]
macro_rules! stdout {
  ($($arg:tt)*) => {{
    use ::std::io::Write;

    match writeln!(&mut std::io::stdout().lock(), $($arg)*) {
      Ok(_) => (),
      Err(err) => panic!("Unable to write to stdout: {}", err)
    }
  }}
}

#[macro_export]
macro_rules! stdout_inline {
  ($($arg:tt)*) => {{
    use ::std::io::Write;

    match write!(&mut std::io::stdout().lock(), $($arg)*) {
      Ok(_) => match std::io::stdout().lock().flush() {
        Ok(_) => (),
        Err(err) => panic!("Unable to flush stdout: {}", err)
      }
      Err(err) => panic!("Unable to write to stdout: {}", err)
    }
  }}
}

#[macro_export]
macro_rules! stderr {
  ($($arg:tt)*) => {
    eprintln!($($arg)*)
  }
}

pub use stderr;
pub use stdout;
pub use stdout_inline;

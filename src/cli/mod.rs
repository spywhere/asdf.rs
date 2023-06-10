pub use clap::Parser;

pub mod options;

mod commands;

pub use commands::Commands;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

pub struct Exit {
  pub exit_code: i32,
  pub message: Option<String>,
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

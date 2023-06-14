use asdf::cli::stdout;
use asdf::cli::{Parser, Cli, Commands, Exit};

use asdf::cmd;

fn main() {
  let cli: Cli = Cli::parse();

  let result = match &cli.command {
    Commands::Plugin(_) => Ok(()),
    Commands::List(_) => cmd::list_all(),
    _ => Err(Exit { exit_code: 1, message: Some("Command not available".to_string()) })
  };

  match result {
    Err(exit) => {
      if let Some(message) = exit.message {
        match exit.exit_code {
          0 => stdout!("{}", message),
          _ => stdout!("ERROR: {}", message)
        }
      }
      std::process::exit(exit.exit_code)
    },
    _ => {},
  }
}

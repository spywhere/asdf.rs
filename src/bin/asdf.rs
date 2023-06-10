use asdf::cli::stdout;
use asdf::cli::{Parser, Cli, Commands, Exit};

fn main() {
  let cli: Cli = Cli::parse();

  let result = match &cli.command {
    Commands::Plugin(_) => Ok(()),
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

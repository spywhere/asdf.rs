use asdf::cli::stdout;
use asdf::cli::{Parser, Cli, Commands, Exit};
use asdf::cli::options;

use asdf::cmd;

fn main() {
  let cli: Cli = Cli::parse();

  let context = cli.envs.into();
  let result = match &cli.command {
    Commands::Plugin(_) => Ok(()),
    Commands::List(cmd) => match (cmd.command.clone(), cmd.list.clone()) {
      (Some(options::ListCommands::All(args)), _) => cmd::list_all(context, args.into()),
      (_, Some(list)) => cmd::list(context, list.into()),
      _ => Err(Exit { exit_code: 1, message: Some("Command not available".to_string()) })
    },
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

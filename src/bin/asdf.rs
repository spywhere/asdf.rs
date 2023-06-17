use asdf::cli::options;
use asdf::cli::stdout;
use asdf::cli::{Cli, Commands, Exit, Parser};

use asdf::cmd;

fn main() {
  let cli: Cli = Cli::parse();

  let context = cli.envs.into();
  let result = match &cli.command {
    Commands::Plugin(_) => Ok(()),
    Commands::List(cmd) => match cmd.command.clone() {
      Some(options::ListCommands::All(args)) => {
        cmd::list_all(context, args.into())
      }
      None => cmd::list(context, cmd.list.clone().into()),
    },
    _ => Err(Exit {
      exit_code: 1,
      message: Some("Command not available".to_string()),
    }),
  };

  if let Err(exit) = result {
    if let Some(message) = exit.message {
      match exit.exit_code {
        0 => stdout!("{}", message),
        _ => stdout!("ERROR: {}", message),
      }
    }
    std::process::exit(exit.exit_code)
  }
}

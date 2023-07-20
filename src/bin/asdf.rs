use asdf::cli::options;
use asdf::cli::stdout;
use asdf::cli::{Cli, Commands, Exit, Parser};

use asdf::cmd;

fn main() {
  let cli: Cli = Cli::parse();

  let context = cli.envs.into();
  let result = match &cli.command {
    Commands::Plugin(cmd) => match cmd.command.clone() {
      options::PluginCommands::List(args) => match args.command {
        Some(_) => cmd::plugin::list_all(context),
        None => cmd::plugin::list(context, args.into())
      },
      _ => Ok(())
    },
    Commands::List(cmd) => match cmd.command.clone() {
      Some(options::ListCommands::All(args)) => {
        cmd::version::list_all(context, args.into())
      }
      None => cmd::version::list(context, cmd.list.clone().into()),
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

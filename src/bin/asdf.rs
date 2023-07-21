use asdf::cli::options;
use asdf::cli::stdout;
use asdf::cli::{Cli, Commands, Parser};

use asdf::cmd;

fn main() {
  let cli: Cli = Cli::parse();

  let context = cli.envs.into();
  let result = match &cli.command {
    Commands::Plugin(cmd) => match cmd.command.clone() {
      options::PluginCommands::Add(args) => {
        cmd::plugin::add(context, args.into())
      }
      options::PluginCommands::List(args) => match args.command {
        Some(_) => cmd::plugin::list_all(context),
        None => cmd::plugin::list(context, args.into()),
      },
      options::PluginCommands::Remove(args) => {
        cmd::plugin::remove(context, args.into())
      }
      options::PluginCommands::Update(args) => {
        cmd::plugin::update(context, args.into())
      }
    },
    Commands::Latest(args) => {
      cmd::package::latest(context, args.clone().into())
    }
    Commands::Install(args) => {
      cmd::package::install(context, args.clone().into())
    }
    Commands::Uninstall(args) => {
      cmd::package::uninstall(context, args.clone().into())
    }
    Commands::Current(args) => {
      cmd::version::current(context, args.clone().into())
    }
    Commands::List(cmd) => match cmd.command.clone() {
      Some(options::ListCommands::All(args)) => {
        cmd::version::list_all(context, args.into())
      }
      None => cmd::version::list(context, cmd.list.clone().into()),
    },
    Commands::Global(args) => {
      cmd::version::global(context, args.clone().into())
    }
    Commands::Local(args) => cmd::version::local(context, args.clone().into()),
    Commands::Shell(args) => cmd::version::shell(context, args.clone().into()),
    Commands::Where(args) => {
      cmd::install::cmd_where(context, args.clone().into())
    }
    Commands::Which(args) => cmd::install::which(context, args.clone().into()),
    Commands::Exec(args) => cmd::install::exec(context, args.clone().into()),
    Commands::Env(args) => cmd::install::env(context, args.clone().into()),
    Commands::Reshim(args) => {
      cmd::install::reshim(context, args.clone().into())
    }
    Commands::ShimVersions(args) => {
      cmd::install::shim(context, args.clone().into())
    }
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

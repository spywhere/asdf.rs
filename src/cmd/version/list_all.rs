use crate::util;

use crate::cli::stdout;
use crate::cli::Exit;

use crate::cmd::Context;

pub struct ListAllOptions {
  pub plugin: String,
  pub version: Option<String>,
}

pub fn list_all(
  context: Context,
  options: ListAllOptions,
) -> Result<(), Exit> {
  let plugin_dir = util::path::get(
    &context,
    util::path::CommonPath::Plugin(options.plugin.clone()),
  );

  if matches!(plugin_dir, None) {
    return Err(Exit {
      exit_code: 1,
      message: Some(format!("No such plugin: {}", options.plugin)),
    });
  };

  let list_bin = util::path::get(
    &context,
    util::path::CommonPath::PluginBinary {
      plugin: options.plugin.clone(),
      binary: "list-all".to_string(),
    },
  );

  let Some(list_bin) = list_bin else {
    return Err(Exit {
      exit_code: 1,
      message: Some(format!("Plugin {} is corrupted (list-bin expected)", options.plugin)),
    });
  };

  stdout!("{}", list_bin.to_str().unwrap());

  Ok(())
}

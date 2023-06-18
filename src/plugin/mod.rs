mod package;
pub use package::*;

use std::path::PathBuf;

use crate::util;

use crate::cmd::Context;

pub static ENTRY_POINT: &str = "asdf.lua";

pub enum PluginError {
  NotFound(String),
  Corrupted { name: String, binary: String },
}

#[derive(Clone)]
pub struct Plugin {
  pub name: String,
  pub(super) plugin_dir: PathBuf,
  pub(super) data_dir: String,
}

pub fn get(context: &Context, name: String) -> Result<Plugin, PluginError> {
  let plugin_dir = util::path::get(
    &context.data_dir,
    util::path::CommonPath::Plugin(name.clone()),
  );

  let Some(plugin_dir) = plugin_dir else {
    return Err(PluginError::NotFound(name));
  };

  Ok(Plugin {
    name,
    plugin_dir,
    data_dir: context.data_dir.clone(),
  })
}

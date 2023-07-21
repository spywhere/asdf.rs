use std::fs;
use std::path::PathBuf;

mod package;
pub use package::*;

use crate::util;

use crate::cmd::Context;

pub static ENTRY_POINT: &str = "asdf.lua";

pub enum PluginError {
  NoPlugin,
  FetchError,
  NotFound(String),
  Corrupted { name: String, binary: String },
}

#[derive(Clone)]
pub struct Plugin {
  pub name: String,
  pub(super) plugin_dir: PathBuf,
  pub(super) data_dir: String,
}

pub fn plugins(context: &Context) -> Result<Vec<Plugin>, PluginError> {
  let plugin_dir =
    util::path::get(&context.data_dir, util::path::CommonPath::Plugin(None));

  let plugin_dir = plugin_dir.ok_or(PluginError::NoPlugin)?;

  let entries =
    fs::read_dir(plugin_dir).map_err(|_| PluginError::FetchError)?;

  let mut plugins: Vec<Plugin> = entries
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_dir())
    .map(|e| e.file_name().into_string().unwrap_or_default())
    .map(|name| get(context, name))
    .filter_map(|p| p.ok())
    .collect();

  plugins.sort_by(|a, b| a.name.cmp(&b.name));

  Ok(plugins)
}

pub fn get(context: &Context, name: String) -> Result<Plugin, PluginError> {
  let plugin_dir = util::path::get(
    &context.data_dir,
    util::path::CommonPath::Plugin(Some(name.clone())),
  );

  let plugin_dir = plugin_dir.ok_or(PluginError::NotFound(name.clone()))?;

  Ok(Plugin {
    name,
    plugin_dir,
    data_dir: context.data_dir.clone(),
  })
}

use mlua::Lua;

mod api;

use crate::util;

pub static INFERRED_ENTRY_CODE: &str = include_str!("../../runtime/asdf.lua");

pub enum EntryPoint {
  Main,
}

pub enum ExecutionError {
  LoadingError,
  InvalidSyntax(String),
  RuntimeError(String),
}

pub trait PluginExecutable {
  fn execute(&self, entrypoint: EntryPoint) -> Result<(), ExecutionError>;
}

impl From<mlua::Error> for ExecutionError {
  fn from(value: mlua::Error) -> Self {
    match value {
      mlua::Error::SyntaxError { message, .. } => {
        ExecutionError::InvalidSyntax(message)
      }
      _ => ExecutionError::RuntimeError(value.to_string()),
    }
  }
}

use crate::plugin::Plugin;
impl PluginExecutable for Plugin {
  fn execute(&self, entrypoint: EntryPoint) -> Result<(), ExecutionError> {
    let entry =
      util::path::check_exists(self.plugin_dir.join("asdf.lua"), false);

    let lua = Lua::new();

    api::load(&lua)?;

    let function = match entry {
      Some(path) => lua.load(path.as_path()).into_function(),
      None => lua
        .load(INFERRED_ENTRY_CODE)
        .set_name("asdf.lua")
        .map_err(|_| ExecutionError::LoadingError)?
        .into_function(),
    }?;

    function.call(())?;

    Ok(())
  }
}

use mlua::Lua;
use std::error::Error;
use std::fmt;

mod api;

use crate::util;

mod validate;
pub use validate::*;

pub static INFERRED_ENTRY_CODE: &str = include_str!("../../runtime/asdf.lua");

#[derive(Clone)]
pub enum EntryPoint {
  Main,
}

impl From<EntryPoint> for String {
  fn from(value: EntryPoint) -> Self {
    match value {
      EntryPoint::Main => "main".to_string()
    }
  }
}

#[derive(Debug)]
pub enum ExecutionError {
  LoadingError,
  InvalidSyntax(String),
  RuntimeError(String),
  EntryPointError(String),
}

#[derive(Debug)]
pub enum LengthRequirement {
  Exact(u8),
  Minimum(u8),
  Between(u8, u8),
  Maximum(u8),
}

#[derive(Debug)]
pub enum RuntimeError {
  ExpectError { expect: String, actual: String },
  ArgumentRequired(String),
  LengthRequired(String, LengthRequirement),
}

impl Error for ExecutionError {}
impl Error for RuntimeError {}

impl fmt::Display for ExecutionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ExecutionError::LoadingError => write!(f, "error while loading default plugin"),
      ExecutionError::InvalidSyntax(message) => write!(f, "{message}"),
      ExecutionError::RuntimeError(message) => write!(f, "{message}"),
      ExecutionError::EntryPointError(entry) => write!(f, "entry point '{entry}' is missing"),
    }
  }
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RuntimeError::ExpectError { expect, actual } => {
        write!(f, "expected {expect}, got {actual} instead")
      }
      RuntimeError::ArgumentRequired(name) => {
        write!(f, "argument '{name}' is required")
      }
      RuntimeError::LengthRequired(name, requirement) => match requirement {
        LengthRequirement::Exact(size) => {
          write!(f, "expected '{name}' to have a length of {size}")
        }
        LengthRequirement::Minimum(size) => {
          write!(f, "expected '{name}' to have a at least a length of {size}")
        }
        LengthRequirement::Between(min, max) => {
          write!(
            f,
            "expected '{name}' to have a length between {min} and {max}"
          )
        }
        LengthRequirement::Maximum(size) => {
          write!(f, "expected '{name}' to have a at most a length of {size}")
        }
      },
    }
  }
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

use crate::plugin::{self, Plugin};
impl PluginExecutable for Plugin {
  fn execute(&self, entrypoint: EntryPoint) -> Result<(), ExecutionError> {
    let entry =
      util::path::check_exists(self.plugin_dir.join(plugin::ENTRY_POINT), false);

    let lua = Lua::new();

    let function = match entry {
      Some(path) => lua.load(path.as_path()).into_function(),
      None => lua
        .load(INFERRED_ENTRY_CODE)
        .set_name("asdf.lua")
        .map_err(|_| ExecutionError::LoadingError)?
        .into_function(),
    }?;

    let module: mlua::Value = function.call(())?;
    let module: mlua::Table = unwrap_expect(&lua, module)?;

    api::load(self, &lua)?;

    let Some(entry) = module.try_get(entrypoint.clone().into())? else {
      return Err(ExecutionError::EntryPointError(entrypoint.into()));
    };

    let entry: mlua::Function = unwrap_expect(&lua, entry)?;
    entry.call(())?;

    Ok(())
  }
}

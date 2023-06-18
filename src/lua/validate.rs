use super::RuntimeError;

pub trait TableExpectation {
  fn expect_get(&self, key: String) -> Result<mlua::Value, mlua::Error>;
  fn try_get(&self, key: String) -> Result<Option<mlua::Value>, mlua::Error>;
}

impl TableExpectation for mlua::Table<'_> {
  fn expect_get(&self, key: String) -> Result<mlua::Value, mlua::Error> {
    if !self.contains_key(key.clone())? {
      return Err(mlua::Error::external(RuntimeError::ArgumentRequired(key)));
    }

    self.get(key)
  }

  fn try_get(&self, key: String) -> Result<Option<mlua::Value>, mlua::Error> {
    if self.contains_key(key.clone())? {
      Ok(Some(self.get(key)?))
    } else {
      Ok(None)
    }
  }
}

#[macro_export]
macro_rules! unwrap_expect {
  ($var:expr, $kind:path) => {{
    let value = $var;
    let $kind(value) = value else {
      return Err(mlua::Error::external($crate::lua::RuntimeError::ExpectError {
        /// TODO: fix incorrect type detection
        expect: mlua::Value::type_name(&value).to_string(),
        actual: value.type_name().to_string(),
      }))
    };
    Ok::<_, mlua::Error>(value)
  }};
}
pub use unwrap_expect;

#[macro_export]
macro_rules! try_expect {
  ($var:expr, $kind:path) => {{
    let value = $var;
    match value {
      $kind(value) => Ok::<_, mlua::Error>(Some(value)),
      mlua::Value::Nil => Ok::<_, mlua::Error>(None),
      _ => Err(mlua::Error::external($crate::lua::RuntimeError::ExpectError {
        expect: mlua::Value::type_name(&value).to_string(),
        actual: value.type_name().to_string(),
      }))
    }
  }};
}
pub use try_expect;

use super::RuntimeError;

pub trait TableExpectation {
  fn try_get(&self, key: String) -> Result<mlua::Value, mlua::Error>;
}

impl TableExpectation for mlua::Table<'_> {
  fn try_get(&self, key: String) -> Result<mlua::Value, mlua::Error> {
    if !self.contains_key(key.clone())? {
      return Err(mlua::Error::external(RuntimeError::ArgumentRequired(key)));
    }

    self.get(key)
  }
}

#[macro_export]
macro_rules! try_expect {
  ($var:expr, $kind:path) => {{
    let value = $var;
    let $kind(value) = value else {
      return Err(mlua::Error::external($crate::lua::RuntimeError::ExpectError {
        expect: mlua::Value::type_name(&value).to_string(),
        actual: value.type_name().to_string(),
      }))
    };
    Ok::<_, mlua::Error>(value)
  }};
}

pub use try_expect;

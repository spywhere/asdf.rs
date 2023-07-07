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

pub fn unwrap_expect<'a, V>(
  lua: &'a mlua::Lua,
  value: mlua::Value<'a>,
) -> Result<V, mlua::Error>
where
  V: mlua::FromLua<'a>,
{
  V::from_lua(value, lua).map_err(|e| match e {
    mlua::Error::FromLuaConversionError { from, to, .. } => {
      mlua::Error::external(crate::lua::RuntimeError::ExpectError {
        expect: to.to_string(),
        actual: from.to_string(),
      })
    }
    _ => e,
  })
}

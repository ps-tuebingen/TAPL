use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::{False, If, True},
};
use common::Eval;

impl Eval<'_> for True {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        Ok(Value::True)
    }
}

impl Eval<'_> for False {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        Ok(Value::False)
    }
}

impl Eval<'_> for If {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        let if_val = self.ifc.clone().eval(_env)?;
        match if_val {
            Value::True => self.thenc.eval(_env),
            Value::False => self.elsec.eval(_env),
            val => Err(Error::eval(ErrorKind::value(&val, "Boolean Value"), &self)),
        }
    }
}

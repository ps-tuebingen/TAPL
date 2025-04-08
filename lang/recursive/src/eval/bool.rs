use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::{False, If, True},
};
use common::Eval;

impl<'a> Eval<'a> for True {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::True)
    }
}

impl<'a> Eval<'a> for False {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::False)
    }
}

impl<'a> Eval<'a> for If {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let cond_val = self.ifc.clone().eval(_env)?;
        match cond_val {
            Value::True => self.thenc.clone().eval(_env),
            Value::False => self.elsec.clone().eval(_env),
            _ => Err(Error::eval(
                ErrorKind::value(cond_val, "Boolean Value"),
                &self,
            )),
        }
    }
}

use super::{Error, Value};
use crate::syntax::{Const, Pred, Succ};
use common::Eval;

impl Eval<'_> for Const {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::Const(self.i))
    }
}

impl Eval<'_> for Succ {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let arg_val = self.term.clone().eval(_env)?;
        let num = arg_val.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num + 1))
    }
}

impl Eval<'_> for Pred {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let arg_val = self.term.clone().eval(_env)?;
        let num = arg_val.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num - 1))
    }
}

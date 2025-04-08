use super::Value;
use crate::{
    errors::Error,
    terms::{IsZero, Pred, Succ, Zero},
};
use common::Eval;

impl<'a> Eval<'a> for Zero {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::Const(0))
    }
}

impl<'a> Eval<'a> for Succ {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num + 1))
    }
}
impl<'a> Eval<'a> for Pred {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num - 1))
    }
}

impl<'a> Eval<'a> for IsZero {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(|knd| Error::eval(knd, &self))?;
        if num == 0 {
            Ok(Value::True)
        } else {
            Ok(Value::False)
        }
    }
}

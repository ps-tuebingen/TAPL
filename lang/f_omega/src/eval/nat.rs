use super::Value;
use crate::{
    errors::Error,
    syntax::terms::{IsZero, Pred, Succ, Zero},
};
use common::Eval;

impl Eval<'_> for Zero {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::Const(0))
    }
}

impl Eval<'_> for Succ {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let inner = self.term.clone().eval(_env)?;
        let i = inner.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(i + 1))
    }
}

impl Eval<'_> for Pred {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let inner = self.term.clone().eval(_env)?;
        let i = inner.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(i - 1))
    }
}

impl Eval<'_> for IsZero {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let inner = self.term.clone().eval(_env)?;
        let i = inner.as_const().map_err(|knd| Error::eval(knd, &self))?;
        if i == 0 {
            Ok(Value::True)
        } else {
            Ok(Value::False)
        }
    }
}

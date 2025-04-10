use super::Value;
use crate::{
    errors::Error,
    terms::{IsZero, Pred, Succ, Zero},
};
use common::Eval;

impl Eval<'_> for Zero {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::Const(0))
    }
}

impl Eval<'_> for Succ {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num + 1))
    }
}
impl Eval<'_> for Pred {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num - 1))
    }
}

impl Eval<'_> for IsZero {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(|knd| Error::eval(knd, &self))?;
        if num == 0 {
            Ok(Value::True)
        } else {
            Ok(Value::False)
        }
    }
}

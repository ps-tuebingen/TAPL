use super::{to_eval_err, Value};
use crate::terms::{IsZero, Pred, Succ, Zero};
use common::{errors::Error, Eval};

impl Eval<'_> for Zero {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::Const(0))
    }
}

impl Eval<'_> for Succ {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(to_eval_err)?;
        Ok(Value::Const(num + 1))
    }
}
impl Eval<'_> for Pred {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(to_eval_err)?;
        Ok(Value::Const(num - 1))
    }
}

impl Eval<'_> for IsZero {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.clone().eval(_env)?;
        let num = val.into_const().map_err(to_eval_err)?;
        if num == 0 {
            Ok(Value::True)
        } else {
            Ok(Value::False)
        }
    }
}

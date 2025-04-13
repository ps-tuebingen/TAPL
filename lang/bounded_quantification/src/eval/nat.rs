use super::{to_eval_err, Error, Value};
use crate::syntax::{Const, Pred, Succ};
use common::Eval;

impl Eval<'_> for Const {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::Const(self.i))
    }
}

impl Eval<'_> for Succ {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let arg_val = self.term.clone().eval(_env)?;
        let num = arg_val.as_const().map_err(to_eval_err)?;
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
        let arg_val = self.term.clone().eval(_env)?;
        let num = arg_val.as_const().map_err(to_eval_err)?;
        Ok(Value::Const(num - 1))
    }
}

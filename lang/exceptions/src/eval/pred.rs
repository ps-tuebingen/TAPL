use super::{Error, Value};
use crate::syntax::Pred;
use common::Eval;

impl Eval<'_> for Pred {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num()?;
        Ok(Value::Const(num - 1))
    }
}

use super::{to_eval_err, Error, Value};
use crate::syntax::Pred;
use common::Eval;

impl Eval<'_> for Pred {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num().map_err(to_eval_err)?;
        Ok(Value::Const(num - 1))
    }
}

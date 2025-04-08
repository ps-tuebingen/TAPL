use super::{Error, Value};
use crate::syntax::Succ;
use common::Eval;

impl Eval<'_> for Succ {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.eval(env)?;
        let num = val.into_num()?;
        Ok(Value::Const(num + 1))
    }
}

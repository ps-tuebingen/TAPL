use super::{Error, Value};
use crate::syntax::Succ;
use common::Eval;

impl Eval for Succ {
    type Value = Value;
    type Error = Error;
    type Env = ();

    fn eval(self, env: &mut Self::Env) -> Result<Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num()?;
        Ok(Value::Const(num + 1))
    }
}

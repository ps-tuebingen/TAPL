use super::{Error, Value};
use crate::syntax::IsZero;
use common::Eval;

impl<'a> Eval<'a> for IsZero {
    type Value = Value;
    type Error = Error;
    type Env = ();

    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num()?;
        if num == 0 {
            Ok(Value::True)
        } else {
            Ok(Value::False)
        }
    }
}

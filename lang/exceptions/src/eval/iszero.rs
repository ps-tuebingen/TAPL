use super::{to_eval_err, Error, Value};
use crate::syntax::IsZero;
use common::Eval;

impl Eval<'_> for IsZero {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.eval(env)?;
        let num = val.into_num().map_err(to_eval_err)?;
        if num == 0 {
            Ok(Value::True)
        } else {
            Ok(Value::False)
        }
    }
}

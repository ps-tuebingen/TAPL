use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{IsZero, Term};

impl<T> Eval for IsZero<T>
where
    T: Term + Eval,
    True<T>: Into<<T as Term>::Value>,
    False<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num().map_err(to_eval_err)?;
        if num.num == 0 {
            Ok(True::new().into())
        } else {
            Ok(False::new().into())
        }
    }
}

use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Pred, Term};

impl<T> Eval for Pred<T>
where
    T: Term + Eval,
    NumVal<T>: Into<<T as Term>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num().map_err(to_eval_err)?;
        Ok(NumVal::new(num.num - 1).into())
    }
}

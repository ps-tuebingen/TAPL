use crate::{
    to_eval_err,
    values::{Num, ValueGroup},
    Eval,
};
use common::errors::Error;
use syntax::terms::{Succ, Term};

impl<T> Eval for Succ<T>
where
    T: Term + Eval,
    Num<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num().map_err(to_eval_err)?;
        Ok(Num::new(num.num + 1).into())
    }
}

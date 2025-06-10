use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    terms::{Succ, Term},
    values::{Num, ValueGroup},
};

impl<T> Eval for Succ<T>
where
    T: Term + Eval,
    Num<T>: Into<<T as Eval>::Value>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let val = self.term.eval(env)?;
        let num = val.into_num()?;
        Ok(Num::new(num.num + 1).into())
    }
}

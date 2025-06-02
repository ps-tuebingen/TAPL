use crate::{
    errors::ValueMismatch,
    values::{Num, ValueGroup},
    Eval,
};
use syntax::terms::{Pred, Term};

impl<T> Eval for Pred<T>
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
        Ok(Num::new(num.num - 1).into())
    }
}

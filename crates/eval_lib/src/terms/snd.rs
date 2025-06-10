use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    terms::{Snd, Term},
    values::ValueGroup,
};

impl<T> Eval for Snd<T>
where
    T: Term + Eval,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        let pair_val = term_val.into_pair()?;
        Ok(*pair_val.snd)
    }
}

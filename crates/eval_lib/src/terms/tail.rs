use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    terms::{Tail, Term},
    types::Type,
    values::ValueGroup,
};

impl<T, Ty> Eval for Tail<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        let cons_val = term_val.into_cons()?;
        Ok(*cons_val.head)
    }
}

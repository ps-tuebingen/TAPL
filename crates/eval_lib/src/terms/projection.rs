use crate::Eval;
use common::errors::{IndexOutOfBounds, ValueMismatch};
use syntax::{
    terms::{Projection, Term},
    values::ValueGroup,
};

impl<T> Eval for Projection<T>
where
    T: Term + Eval,
    <T as Eval>::EvalError: From<ValueMismatch> + From<IndexOutOfBounds>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        let tup_val = term_val.into_tuple()?;
        tup_val
            .vals
            .get(self.index)
            .cloned()
            .ok_or(IndexOutOfBounds::new(self.index, tup_val.vals.len()).into())
    }
}

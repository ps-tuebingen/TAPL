use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    terms::{Term, Unfold},
    types::Type,
    values::ValueGroup,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Unfold<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        let term_fold = term_val.into_fold()?;
        Ok(*term_fold.val)
    }
}

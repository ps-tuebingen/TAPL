use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    subst::SubstTerm,
    terms::{Fix, Term},
    values::ValueGroup,
};

impl<T> Eval for Fix<T>
where
    T: Term + Eval + SubstTerm<T, Target = T>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    Self: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.clone().eval(env)?;
        let lam_val = term_val.into_lambda()?;
        lam_val.body.subst(&lam_val.var, &self.into()).eval(env)
    }
}

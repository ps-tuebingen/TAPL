use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    subst::SubstTerm,
    terms::{Fix, Term},
    values::ValueGroup,
};
use trace::EvalTrace;

impl<T> Eval for Fix<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    Self: Into<T>,
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
        let term_res = self.term.clone().eval(env)?;
        let term_val = term_res.val();
        let lam_val = term_val.into_lambda()?;

        let mut steps = term_res.congruence(&move |t| Fix::new(t).into());

        let body_subst = lam_val.body.subst(&lam_val.var, &self.into());
        let body_res = body_subst.eval(env)?;
        let body_val = body_res.val();
        steps.extend(body_res.steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, body_val))
    }
}

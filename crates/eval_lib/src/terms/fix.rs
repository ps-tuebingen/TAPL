use crate::{errors::EvalError, Eval};

use syntax::{
    store::Store,
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
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
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

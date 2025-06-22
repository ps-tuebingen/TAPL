use crate::{errors::EvalError, Eval};
use syntax::{
    eval_context::EvalContext,
    terms::{Deref, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Deref<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Deref<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.clone().eval(env)?;
        let term_val = term_res.val();
        let loc_val = term_val.clone().into_loc()?;

        let loc_val = env.get_location(loc_val.loc)?;
        let last_step = EvalStep::deref(term_val, loc_val.clone());
        let mut steps = term_res.congruence(&move |t| Deref::new(t).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, loc_val))
    }
}

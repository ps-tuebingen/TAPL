use crate::{errors::EvalError, Eval};

use syntax::{
    eval_context::EvalContext,
    terms::{Snd, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Snd<T>
where
    T: Term + Eval<Term = T>,
    Snd<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let pair_val = term_val.clone().into_pair()?;
        let val = *pair_val.snd;

        let mut steps = term_res.congruence(&move |t| Snd::new(t).into());
        let last_step = EvalStep::snd(Snd::new(term_val), val.clone());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}

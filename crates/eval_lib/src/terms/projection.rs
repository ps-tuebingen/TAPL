use crate::{Eval, errors::EvalError};
use common::errors::IndexOutOfBounds;
use syntax::{
    eval_context::EvalContext,
    terms::{Projection, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Projection<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Projection<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let tup_val = term_val.clone().into_tuple()?;
        let val = tup_val
            .vals
            .get(self.index)
            .cloned()
            .ok_or(IndexOutOfBounds::new(self.index, tup_val.vals.len()))?;

        let mut steps = term_res.congruence(&move |t| Projection::new(t, self.index).into());
        let last_step = EvalStep::projection(Projection::new(term_val, self.index), val.clone());
        steps.push(last_step);

        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}

use crate::Eval;
use common::errors::{UndefinedLocation, ValueMismatch};
use syntax::{
    store::Store,
    terms::{Deref, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Deref<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Deref<T>: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch> + From<UndefinedLocation>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
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

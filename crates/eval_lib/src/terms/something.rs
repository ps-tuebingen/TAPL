use crate::{Eval, errors::EvalError};
use syntax::{
    eval_context::EvalContext,
    terms::{Something, Term},
    values::Something as SomethingVal,
};
use trace::EvalTrace;

impl<T> Eval for Something<T>
where
    T: Term + Eval<Term = T>,
    Something<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    SomethingVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val = SomethingVal::<<T as Eval>::Value>::new(term_val);
        let steps = term_res.congruence(&move |t| Something::new(t).into());
        Ok(EvalTrace::new(steps, val))
    }
}

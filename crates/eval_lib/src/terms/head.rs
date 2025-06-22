use crate::{errors::EvalError, Eval};

use syntax::{
    store::Store,
    terms::{Head, Term},
    types::Type,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Head<T, Ty>
where
    T: Term + Eval<Term = T>,
    Ty: Type,
    Head<T, Ty>: Into<T>,
    <T as Eval>::Value: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let cons_val = term_val.clone().into_cons()?;

        let last_step =
            EvalStep::head(Head::new(term_val, self.ty.clone()), *cons_val.head.clone());
        let mut steps = term_res.congruence(&move |t| Head::new(t, self.ty.clone()).into());
        steps.push(last_step);

        Ok(EvalTrace::<T, <T as Eval>::Value>::new(
            steps,
            *cons_val.head,
        ))
    }
}

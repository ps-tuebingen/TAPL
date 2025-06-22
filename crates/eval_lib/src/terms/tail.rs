use crate::{errors::EvalError, Eval};

use syntax::{
    store::Store,
    terms::{Tail, Term},
    types::Type,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Tail<T, Ty>
where
    T: Term + Eval<Term = T>,
    Tail<T, Ty>: Into<T>,
    <T as Eval>::Value: Into<T>,
    Ty: Type,
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

        let val = *cons_val.head;
        let last_step = EvalStep::tail(Tail::new(term_val, self.ty.clone()), val.clone());
        let mut steps = term_res.congruence(&move |t| Tail::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}

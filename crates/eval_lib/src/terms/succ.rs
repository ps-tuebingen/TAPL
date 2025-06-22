use crate::{Eval, errors::EvalError};

use syntax::{
    store::Store,
    terms::{Num as NumT, Succ, Term},
    values::{Num, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Succ<T>
where
    T: Term + Eval<Term = T>,
    Num<T>: Into<<T as Eval>::Value>,
    Succ<T>: Into<T>,
    NumT<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let num = term_val.into_num()?;
        let last_step = EvalStep::succ(num.num);
        let mut steps = term_res.congruence(&move |t| Succ::new(t).into());
        steps.push(last_step);
        let val = Num::<T>::new(num.num + 1);
        Ok(EvalTrace::new(steps, val))
    }
}

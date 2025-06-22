use crate::{Eval, errors::EvalError};

use syntax::{
    store::Store,
    terms::{Num as NumT, Pred, Term},
    values::{Num, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Pred<T>
where
    T: Term + Eval<Term = T>,
    Pred<T>: Into<T>,
    NumT<T>: Into<T>,
    <T as Eval>::Value: Into<T>,
    Num<T>: Into<<T as Eval>::Value>,
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
        let val = Num::<T>::new(num.num - 1);
        let mut steps = term_res.congruence(&move |t| Pred::new(t).into());
        let last_step = EvalStep::pred(num.num);
        steps.push(last_step);
        Ok(EvalTrace::new(steps, val))
    }
}

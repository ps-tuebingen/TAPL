use crate::{Eval, errors::EvalError};
use syntax::{
    eval_context::EvalContext,
    terms::{Num, Term},
    values::Num as NumVal,
};
use trace::EvalTrace;

impl<T> Eval for Num<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    NumVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(vec![], NumVal::new(self.num)))
    }
}

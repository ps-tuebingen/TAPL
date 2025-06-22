use crate::{Eval, errors::EvalError};
use syntax::{
    store::Store,
    terms::{False, Term},
    values::False as FalseVal,
};
use trace::EvalTrace;

impl<T> Eval for False<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    FalseVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(vec![], FalseVal::new()))
    }
}

use crate::Eval;
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
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(EvalTrace::new(vec![], FalseVal::new()))
    }
}

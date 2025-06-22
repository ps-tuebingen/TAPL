use crate::Eval;
use syntax::{
    store::Store,
    terms::{Term, True},
    values::True as TrueVal,
};
use trace::EvalTrace;

impl<T> Eval for True<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    TrueVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(EvalTrace::new(vec![], TrueVal::new()))
    }
}

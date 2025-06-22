use crate::{errors::EvalError, Eval};
use syntax::{
    store::Store,
    terms::{Term, Unit},
    values::Unit as UnitVal,
};
use trace::EvalTrace;

impl<T> Eval for Unit<T>
where
    T: Term + Eval<Term = T>,
    UnitVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(vec![], UnitVal::new()))
    }
}

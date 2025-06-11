use crate::Eval;
use syntax::{
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
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(EvalTrace::new(vec![], UnitVal::new()))
    }
}

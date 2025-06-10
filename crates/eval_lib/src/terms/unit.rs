use crate::Eval;
use syntax::{
    terms::{Term, Unit},
    values::Unit as UnitVal,
};

impl<T> Eval for Unit<T>
where
    T: Term + Eval,
    UnitVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(UnitVal::new().into())
    }
}

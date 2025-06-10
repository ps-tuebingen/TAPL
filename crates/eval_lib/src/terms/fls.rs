use crate::Eval;
use syntax::{
    terms::{False, Term},
    values::False as FalseVal,
};
use trace::EvalTrace;

impl<T> Eval for False<T>
where
    T: Term + Eval,
    FalseVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(FalseVal::new().into())
    }
}

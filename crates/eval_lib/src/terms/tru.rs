use crate::Eval;
use syntax::{
    terms::{Term, True},
    values::True as TrueVal,
};

impl<T> Eval for True<T>
where
    T: Term + Eval,
    TrueVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(TrueVal::new().into())
    }
}

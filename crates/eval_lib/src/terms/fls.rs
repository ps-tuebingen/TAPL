use crate::{values::False as FalseVal, Eval};
use syntax::terms::{False, Term};

impl<T> Eval for False<T>
where
    T: Term + Eval,
    FalseVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(FalseVal::new().into())
    }
}

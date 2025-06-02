use crate::{values::True as TrueVal, Eval};
use syntax::terms::{Term, True};

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

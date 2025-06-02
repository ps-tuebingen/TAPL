use crate::{
    errors::ValueMismatch,
    values::{False, True, ValueGroup},
    Eval,
};
use syntax::terms::{IsZero, Term};

impl<T> Eval for IsZero<T>
where
    T: Term + Eval,
    True<T>: Into<<T as Eval>::Value>,
    False<T>: Into<<T as Eval>::Value>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let val = self.term.eval(env)?;
        let num = val.into_num()?;
        if num.num == 0 {
            Ok(True::new().into())
        } else {
            Ok(False::new().into())
        }
    }
}

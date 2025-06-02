use crate::{
    errors::{ValueKind, ValueMismatch},
    values::{False, True, ValueGroup},
    Eval,
};
use syntax::{
    terms::{IsNil, Term},
    types::Type,
};

impl<T, Ty> Eval for IsNil<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    True<T>: Into<<T as Eval>::Value>,
    False<T>: Into<<T as Eval>::Value>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        if term_val.clone().into_nil().is_ok() {
            Ok(True::new().into())
        } else if term_val.clone().into_cons().is_ok() {
            Ok(False::new().into())
        } else {
            Err(ValueMismatch::new(&term_val, ValueKind::List).into())
        }
    }
}

use crate::Eval;
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    terms::{IsNil, Term},
    types::Type,
    values::{False, True, Value, ValueGroup},
};
use trace::EvalTrace;

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

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        if term_val.clone().into_nil().is_ok() {
            Ok(True::new().into())
        } else if term_val.clone().into_cons().is_ok() {
            Ok(False::new().into())
        } else {
            Err(ValueMismatch::new(term_val.knd(), ValueKind::List).into())
        }
    }
}

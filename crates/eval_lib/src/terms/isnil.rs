use crate::{
    to_eval_err,
    values::{False, True, ValueGroup},
    Eval,
};
use common::errors::{Error, ErrorKind};
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
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        if term_val.clone().into_nil().is_ok() {
            Ok(True::new().into())
        } else if term_val.clone().into_cons().is_ok() {
            Ok(False::new().into())
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: term_val.to_string(),
                expected: "List Value".to_owned(),
            }))
        }
    }
}

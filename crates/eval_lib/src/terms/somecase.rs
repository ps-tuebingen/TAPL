use crate::Eval;
use common::errors::Error;
use syntax::terms::{SomeCase, Term};

impl<T> Eval for SomeCase<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;

        if let Ok(some_val) = bound_val.clone().into_something() {
            self.some_term
                .subst(&self.some_var, &((*some_val.val).into()))
                .eval(env)
        } else if bound_val.clone().into_nothing().is_ok() {
            self.none_term.eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: bound_val.to_string(),
                expected: "Option Value".to_owned(),
            }))
        }
    }
}

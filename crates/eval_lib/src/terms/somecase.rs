use crate::{
    errors::{ValueKind, ValueMismatch},
    values::ValueGroup,
    Eval,
};
use syntax::{
    subst::SubstTerm,
    terms::{SomeCase, Term},
};

impl<T> Eval for SomeCase<T>
where
    T: Term + Eval + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let bound_val = self.bound_term.eval(env)?;

        if let Ok(some_val) = bound_val.clone().into_something() {
            self.some_term
                .subst(&self.some_var, &((*some_val.val).into()))
                .eval(env)
        } else if bound_val.clone().into_nothing().is_ok() {
            self.none_term.eval(env)
        } else {
            Err(ValueMismatch::new(&bound_val, ValueKind::Option).into())
        }
    }
}

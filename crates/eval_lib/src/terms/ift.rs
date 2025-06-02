use crate::{
    errors::{ValueKind, ValueMismatch},
    values::ValueGroup,
    Eval,
};
use syntax::terms::{If, Term};

impl<T> Eval for If<T>
where
    T: Term + Eval,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let cond_val = self.if_cond.eval(env)?;
        if cond_val.clone().into_true().is_ok() {
            self.then_term.eval(env)
        } else if cond_val.clone().into_false().is_ok() {
            self.else_term.eval(env)
        } else {
            Err(ValueMismatch::new(&cond_val, ValueKind::Bool).into())
        }
    }
}

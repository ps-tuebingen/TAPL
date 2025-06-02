use crate::{
    errors::{ValueKind, ValueMismatch},
    values::ValueGroup,
    Eval,
};
use syntax::{
    subst::SubstTerm,
    terms::{SumCase, Term},
};

impl<T> Eval for SumCase<T>
where
    T: Term + Eval + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let bound_val = self.bound_term.eval(env)?;
        if let Ok(left_val) = bound_val.clone().into_left() {
            self.left_term
                .subst(&self.left_var, &((*left_val.left_val).into()))
                .eval(env)
        } else if let Ok(right_val) = bound_val.clone().into_right() {
            self.right_term
                .subst(&self.right_var, &((*right_val.right_val).into()))
                .eval(env)
        } else {
            Err(ValueMismatch::new(&bound_val, ValueKind::Sum).into())
        }
    }
}

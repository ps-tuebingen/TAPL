use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::{Error, ErrorKind};
use syntax::{
    subst::SubstTerm,
    terms::{SumCase, Term},
};

impl<T> Eval for SumCase<T>
where
    T: Term + Eval + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
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
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: bound_val.to_string(),
                expected: "Sum Value".to_owned(),
            }))
        }
    }
}

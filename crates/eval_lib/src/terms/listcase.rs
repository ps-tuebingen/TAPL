use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::{Error, ErrorKind};
use syntax::terms::{ListCase, Term};

impl<T> Eval for ListCase<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        if bound_val.clone().into_nil().is_ok() {
            self.nil_rhs.eval(env)
        } else if let Ok(cons) = bound_val.clone().into_cons() {
            self.cons_rhs
                .subst(&self.cons_fst, &((*cons.head).into()))
                .subst(&self.cons_rst, &((*cons.tail).into()))
                .eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: bound_val.to_string(),
                expected: "List".to_owned(),
            }))
        }
    }
}

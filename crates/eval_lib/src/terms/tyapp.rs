use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Term, TyApp};

impl<T> Eval for TyApp<T>
where
    T: Term + Eval,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let fun_val = self.fun.eval(env)?;
        if let Ok(tylam) = fun_val.clone().into_tylambda() {
            tylam.term.subst_type(&tylam.var, &self.arg).eval(env)
        } else if let Ok(lamsub) = fun_val.clone().into_lambdasub() {
            lamsub.term.subst_type(&lamsub.var, &self.arg).eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: fun_val.to_string(),
                expected: "Type Abstraction".to_owned(),
            }))
        }
    }
}

use super::{errors::Error, Value};
use crate::syntax::{Subst, TyApp, TyLambda};
use common::Eval;

impl<'a> Eval<'a> for TyLambda {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::TyLambda {
            var: self.var,
            body: *self.term,
        })
    }
}

impl<'a> Eval<'a> for TyApp {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.eval(_env)?;
        if let Value::TyLambda { var, body } = val {
            body.subst_ty(&var, self.ty).eval(_env)
        } else {
            Err(Error::NotATyAbs(val))
        }
    }
}

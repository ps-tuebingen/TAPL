use super::{errors::Error, Value};
use crate::syntax::{Subst, TyApp, TyLambda};
use common::Eval;

impl Eval<'_> for TyLambda {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::TyLambda {
            var: self.var,
            body: *self.term,
        })
    }
}

impl Eval<'_> for TyApp {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.eval(_env)?;
        if let Value::TyLambda { var, body } = val {
            body.subst_ty(&var, self.ty).eval(_env)
        } else {
            Err(Error::NotATyAbs(val))
        }
    }
}

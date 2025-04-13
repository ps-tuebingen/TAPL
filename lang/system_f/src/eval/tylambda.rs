use super::{to_eval_err, Value};
use crate::syntax::{Subst, TyApp, TyLambda};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for TyLambda {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

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

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.eval(_env)?;
        if let Value::TyLambda { var, body } = val {
            body.subst_ty(&var, self.ty).eval(_env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "Type Abstraction".to_owned(),
            }))
        }
    }
}

use super::{errors::Error, Eval, Value};
use crate::syntax::{Subst, TyApp, TyLambda};

impl Eval for TyLambda {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::TyLambda {
            var: self.var,
            body: *self.term,
        })
    }
}

impl Eval for TyApp {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        if let Value::TyLambda { var, body } = val {
            body.subst_ty(&var, self.ty).eval()
        } else {
            Err(Error::NotATyAbs(val))
        }
    }
}

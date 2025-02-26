use super::{Eval, Value};
use crate::{errors::Error, syntax::terms::TyLambda};

impl Eval for TyLambda {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::TyLambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}

use super::Value;
use crate::{errors::Error, syntax::terms::TyLambda};
use common::Eval;

impl Eval<'_> for TyLambda {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::TyLambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}

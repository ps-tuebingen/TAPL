use super::{Env, Value};
use crate::{errors::Error, syntax::terms::TyLambda};
use common::Eval;

impl<'a> Eval<'a> for TyLambda {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        Ok(Value::TyLambda {
            var: self.var,
            sup_ty: self.sup_ty,
            body: *self.body,
        })
    }
}

use super::{Env, Eval, Value};
use crate::{errors::Error, syntax::terms::TyLambda};

impl Eval for TyLambda {
    type Target = Value;
    fn eval(self, _: &mut Env) -> Result<Self::Target, Error> {
        Ok(Value::TyLambda {
            var: self.var,
            sup_ty: self.sup_ty,
            body: *self.body,
        })
    }
}

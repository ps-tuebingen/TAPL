use super::{Env, Value};
use crate::syntax::terms::TyLambda;
use common::{errors::Error, Eval};

impl<'a> Eval<'a> for TyLambda {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::TyLambda {
            var: self.var,
            sup_ty: self.sup_ty,
            body: *self.body,
        })
    }
}

use super::{Env, Value};
use crate::{errors::Error, syntax::terms::Pack};
use common::Eval;

impl<'a> Eval<'a> for Pack {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        let val = self.term.eval(env)?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(val),
            outer_ty: self.outer_ty,
        })
    }
}

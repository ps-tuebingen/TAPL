use super::Value;
use crate::{errors::Error, syntax::terms::Pack};
use common::Eval;

impl Eval<'_> for Pack {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.term.eval(_env)?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(val),
            outer_ty: self.outer_ty,
        })
    }
}

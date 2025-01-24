use super::{Eval, Value};
use crate::{errors::Error, syntax::terms::Pack};

impl Eval for Pack {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        Ok(Value::Pack {
            inner_ty: self.inner_ty,
            val: Box::new(val),
            outer_ty: self.outer_ty,
        })
    }
}

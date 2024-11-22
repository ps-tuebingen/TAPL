use super::{errors::Error, Eval, Value};
use crate::terms::syntax::{Nothing, Something};

impl Eval for Something {
    fn eval(self) -> Result<Value, Error> {
        let val = self.term.eval()?;
        Ok(Value::Something(Box::new(val)))
    }
}

impl Eval for Nothing {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Nothing {
            inner_type: self.inner_type,
        })
    }
}

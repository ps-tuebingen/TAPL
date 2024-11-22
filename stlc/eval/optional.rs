use super::{Eval, Value};
use crate::terms::syntax::{Nothing, Something};

impl Eval for Something {
    fn eval(self) -> Option<Value> {
        let val = self.term.eval()?;
        Some(Value::Something(Box::new(val)))
    }
}

impl Eval for Nothing {
    fn eval(self) -> Option<Value> {
        Some(Value::Nothing {
            inner_type: self.inner_type,
        })
    }
}

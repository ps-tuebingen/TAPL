use super::{errors::Error, Eval, Value};
use crate::syntax::Unit;

impl Eval for Unit {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Unit)
    }
}

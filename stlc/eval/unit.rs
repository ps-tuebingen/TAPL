use super::{Eval, Value};
use crate::terms::syntax::Unit;

impl Eval for Unit {
    fn eval(self) -> Option<Value> {
        Some(Value::Unit)
    }
}

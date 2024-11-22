use super::{Eval, Value};
use crate::terms::syntax::{False, If, True};
impl Eval for True {
    fn eval(self) -> Option<Value> {
        Some(Value::True)
    }
}

impl Eval for False {
    fn eval(self) -> Option<Value> {
        Some(Value::False)
    }
}

impl Eval for If {
    fn eval(self) -> Option<Value> {
        let if_v = self.ifc.eval()?;
        match if_v {
            Value::True => self.thenc.eval(),
            Value::False => self.elsec.eval(),
            _ => None,
        }
    }
}

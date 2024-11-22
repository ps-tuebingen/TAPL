use super::{errors::Error, Eval, Value};
use crate::terms::syntax::{False, If, True};
impl Eval for True {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::True)
    }
}

impl Eval for False {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::False)
    }
}

impl Eval for If {
    fn eval(self) -> Result<Value, Error> {
        let if_v = self.ifc.eval()?;
        match if_v {
            Value::True => self.thenc.eval(),
            Value::False => self.elsec.eval(),
            _ => Err(Error::BadValue { val: if_v }),
        }
    }
}

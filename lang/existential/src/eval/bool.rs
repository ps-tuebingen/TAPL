use super::{Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    terms::{False, If, True},
};

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
        let if_val = self.ifc.clone().eval()?;
        match if_val {
            Value::True => self.thenc.eval(),
            Value::False => self.elsec.eval(),
            val => Err(Error::eval(ErrorKind::value(&val, "Boolean Value"), &self)),
        }
    }
}

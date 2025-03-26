use super::{Error, Eval, Value};
use crate::syntax::{Const, Pred, Succ};

impl Eval for Const {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Const(self.i))
    }
}

impl Eval for Succ {
    fn eval(self) -> Result<Value, Error> {
        let arg_val = self.term.clone().eval()?;
        let num = arg_val.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num + 1))
    }
}

impl Eval for Pred {
    fn eval(self) -> Result<Value, Error> {
        let arg_val = self.term.clone().eval()?;
        let num = arg_val.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(num - 1))
    }
}

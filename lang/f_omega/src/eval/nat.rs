use super::{Eval, Value};
use crate::{
    errors::Error,
    syntax::terms::{IsZero, Pred, Succ, Zero},
};

impl Eval for Zero {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Const(0))
    }
}

impl Eval for Succ {
    fn eval(self) -> Result<Value, Error> {
        let inner = self.term.clone().eval()?;
        let i = inner.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(i + 1))
    }
}

impl Eval for Pred {
    fn eval(self) -> Result<Value, Error> {
        let inner = self.term.clone().eval()?;
        let i = inner.as_const().map_err(|knd| Error::eval(knd, &self))?;
        Ok(Value::Const(i - 1))
    }
}

impl Eval for IsZero {
    fn eval(self) -> Result<Value, Error> {
        let inner = self.term.clone().eval()?;
        let i = inner.as_const().map_err(|knd| Error::eval(knd, &self))?;
        if i == 0 {
            Ok(Value::True)
        } else {
            Ok(Value::False)
        }
    }
}

use super::{Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    terms::{IsZero, Pred, Succ, Zero},
};

impl Eval for Zero {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Zero)
    }
}

impl Eval for Succ {
    fn eval(self) -> Result<Value, Error> {
        let t_evaled = self.0.clone().eval()?;
        match t_evaled {
            Value::Zero => Ok(Value::Succ(Box::new(Value::Zero))),
            Value::Succ(v) => Ok(Value::Succ(Box::new(Value::Succ(v)))),
            Value::Pred(v) => Ok(*v),
            _ => Err(Error::eval(
                ErrorKind::value(&t_evaled, "Natural Number"),
                &self,
            )),
        }
    }
}

impl Eval for Pred {
    fn eval(self) -> Result<Value, Error> {
        let t_evaled = self.0.clone().eval()?;
        match t_evaled {
            Value::Zero => Ok(Value::Pred(Box::new(Value::Zero))),
            Value::Succ(v) => Ok(*v),
            Value::Pred(v) => Ok(Value::Pred(Box::new(Value::Pred(v)))),
            _ => Err(Error::eval(
                ErrorKind::value(&t_evaled, "Natural Number"),
                &self,
            )),
        }
    }
}

impl Eval for IsZero {
    fn eval(self) -> Result<Value, Error> {
        let val = self.0.clone().eval()?;
        match val {
            Value::Zero => Ok(Value::True),
            Value::Succ(_) => Ok(Value::False),
            Value::Pred(_) => Ok(Value::False),
            val => Err(Error::eval(ErrorKind::value(&val, "Natural Number"), &self)),
        }
    }
}

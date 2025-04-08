use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::{IsZero, Pred, Succ, Zero},
};
use common::Eval;

impl Eval<'_> for Zero {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::Zero)
    }
}

impl Eval<'_> for Succ {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let t_evaled = self.0.clone().eval(_env)?;
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

impl Eval<'_> for Pred {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let t_evaled = self.0.clone().eval(_env)?;
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

impl Eval<'_> for IsZero {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let val = self.0.clone().eval(_env)?;
        match val {
            Value::Zero => Ok(Value::True),
            Value::Succ(_) => Ok(Value::False),
            Value::Pred(_) => Ok(Value::False),
            val => Err(Error::eval(ErrorKind::value(&val, "Natural Number"), &self)),
        }
    }
}

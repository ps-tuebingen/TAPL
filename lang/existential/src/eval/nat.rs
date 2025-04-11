use super::{to_eval_err, Value};
use crate::terms::{IsZero, Pred, Succ, Zero};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

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
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: t_evaled.to_string(),
                expected: "Natural Number".to_string(),
            })),
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
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: t_evaled.to_string(),
                expected: "Natural Number".to_owned(),
            })),
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
            val => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: val.to_string(),
                expected: "Natural Number".to_string(),
            })),
        }
    }
}

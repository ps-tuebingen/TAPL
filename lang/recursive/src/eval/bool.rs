use super::{to_eval_err, Value};
use crate::terms::{False, If, True};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for True {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::True)
    }
}

impl Eval<'_> for False {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        Ok(Value::False)
    }
}

impl Eval<'_> for If {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let cond_val = self.ifc.clone().eval(_env)?;
        match cond_val {
            Value::True => self.thenc.clone().eval(_env),
            Value::False => self.elsec.clone().eval(_env),
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: cond_val.to_string(),
                expected: "Boolean Value".to_owned(),
            })),
        }
    }
}

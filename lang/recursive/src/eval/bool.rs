use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::{False, If, True},
};
use common::Eval;

impl Eval<'_> for True {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::True)
    }
}

impl Eval<'_> for False {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::False)
    }
}

impl Eval<'_> for If {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let cond_val = self.ifc.clone().eval(_env)?;
        match cond_val {
            Value::True => self.thenc.clone().eval(_env),
            Value::False => self.elsec.clone().eval(_env),
            _ => Err(Error::eval(
                ErrorKind::value(cond_val, "Boolean Value"),
                &self,
            )),
        }
    }
}

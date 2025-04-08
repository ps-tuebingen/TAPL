use super::{errors::Error, Value};
use crate::syntax::{App, Lambda, Subst};
use common::Eval;

impl Eval<'_> for Lambda {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        Ok(Value::Lambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}

impl Eval<'_> for App {
    type Value = Value;
    type Err = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let fun_val = self.fun.eval(_env)?;
        match fun_val {
            Value::Lambda {
                var,
                annot: _,
                body,
            } => {
                let arg_val = self.arg.eval(_env)?;
                body.subst(&var, arg_val.into()).eval(_env)
            }
            _ => Err(Error::NotAFunction(fun_val)),
        }
    }
}

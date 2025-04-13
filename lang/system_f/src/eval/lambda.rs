use super::{to_eval_err, Value};
use crate::syntax::{App, Lambda, Subst};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for Lambda {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

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

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

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
            _ => Err(to_eval_err(ErrorKind::ValueMismatch {
                found: fun_val.to_string(),
                expected: "Function".to_owned(),
            })),
        }
    }
}

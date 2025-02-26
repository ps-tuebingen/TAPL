use super::{errors::Error, Eval, Value};
use crate::syntax::{App, Lambda, Subst};

impl Eval for Lambda {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Lambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}

impl Eval for App {
    fn eval(self) -> Result<Value, Error> {
        let fun_val = self.fun.eval()?;
        match fun_val {
            Value::Lambda {
                var,
                annot: _,
                body,
            } => {
                let arg_val = self.arg.eval()?;
                body.subst(&var, arg_val.into()).eval()
            }
            _ => Err(Error::NotAFunction(fun_val)),
        }
    }
}

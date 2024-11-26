use super::{errors::Error, Eval, Value};
use crate::{
    syntax::{App, Lambda},
    traits::subst::Subst,
};

impl Eval for Lambda {
    fn eval(self) -> Result<Value, Error> {
        Ok(Value::Lambda {
            var: self.var.clone(),
            annot: self.annot.clone(),
            body: *self.body.clone(),
        })
    }
}

impl Eval for App {
    fn eval(self) -> Result<Value, Error> {
        let val1 = self.fun.eval()?;
        match val1 {
            Value::Lambda {
                var,
                annot: _,
                body,
            } => {
                let body_subst = body.subst(&var, *self.arg);
                body_subst.eval()
            }
            _ => Err(Error::BadValue { val: val1 }),
        }
    }
}

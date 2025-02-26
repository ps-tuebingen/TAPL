use super::{Eval, Value};
use crate::{
    errors::Error,
    syntax::{App, Lambda},
    traits::SubstTerm,
};

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
        let fun_val = self.fun.clone().eval()?;
        let (var, _, body) = fun_val.as_lam().map_err(|knd| Error::check(knd, &self))?;
        body.subst(&var, *self.arg).eval()
    }
}

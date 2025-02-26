use super::{Eval, Value};
use crate::{
    errors::Error,
    terms::{App, Lambda},
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
        let (var, _, body) = self
            .fun
            .clone()
            .eval()?
            .as_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        body.subst(&var, *self.arg).eval()
    }
}

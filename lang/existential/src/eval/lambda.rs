use super::Value;
use crate::{
    errors::Error,
    terms::{App, Lambda},
    traits::SubstTerm,
};
use common::Eval;

impl Eval<'_> for Lambda {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        Ok(Value::Lambda {
            var: self.var,
            annot: self.annot,
            body: *self.body,
        })
    }
}
impl Eval<'_> for App {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Value, Error> {
        let (var, _, body) = self
            .fun
            .clone()
            .eval(_env)?
            .as_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        body.subst(&var, *self.arg).eval(_env)
    }
}

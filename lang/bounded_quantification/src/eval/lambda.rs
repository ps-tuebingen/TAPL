use super::Value;
use crate::{
    errors::Error,
    syntax::{App, Lambda},
    traits::SubstTerm,
};
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
        let fun_val = self.fun.clone().eval(_env)?;
        let (var, _, body) = fun_val.as_lam().map_err(|knd| Error::check(knd, &self))?;
        body.subst(&var, *self.arg).eval(_env)
    }
}

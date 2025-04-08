use super::Value;
use crate::{errors::Error, syntax::terms::App, traits::SubstTerm};
use common::Eval;

impl Eval<'_> for App {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let fun_val = self.fun.clone().eval(_env)?;
        let (var, _, body) = fun_val.as_lambda().map_err(|knd| Error::eval(knd, &self))?;
        body.subst(&var, *self.arg).eval(_env)
    }
}

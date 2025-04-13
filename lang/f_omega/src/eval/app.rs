use super::{to_eval_err, Value};
use crate::{syntax::terms::App, traits::SubstTerm};
use common::{errors::Error, Eval};

impl Eval<'_> for App {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let fun_val = self.fun.clone().eval(_env)?;
        let (var, _, body) = fun_val.as_lambda().map_err(to_eval_err)?;
        body.subst(&var, *self.arg).eval(_env)
    }
}

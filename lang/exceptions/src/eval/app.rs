use super::{to_eval_err, Error, Value};
use crate::{syntax::App, traits::subst::Subst};
use common::Eval;

impl Eval<'_> for App {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let fun_val = self.fun.eval(env)?;
        let arg_val = self.arg.eval(env)?;
        let (var, _, body) = fun_val.into_lambda().map_err(to_eval_err)?;
        body.subst(&var, arg_val.into()).eval(env)
    }
}

use super::{to_eval_err, Env, Value};
use crate::{syntax::terms::App, traits::SubstTerm};
use common::{errors::Error, Eval};

impl<'a> Eval<'a> for App {
    type Value = Value;
    type Err = Error;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let fun_val = self.fun.clone().eval(&mut env.clone())?;
        let (var, _, body) = fun_val.as_lambda().map_err(to_eval_err)?;
        body.subst(&var, *self.arg).eval(env)
    }
}

use super::{Env, Value};
use crate::{errors::Error, syntax::terms::App, traits::SubstTerm};
use common::Eval;

impl<'a> Eval<'a> for App {
    type Value = Value;
    type Err = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let fun_val = self.fun.clone().eval(&mut env.clone())?;
        let (var, _, body) = fun_val
            .as_lambda()
            .map_err(|knd| Error::eval(knd, self.clone()))?;
        body.subst(&var, *self.arg).eval(env)
    }
}

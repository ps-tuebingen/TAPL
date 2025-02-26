use super::{Env, Eval, Value};
use crate::{errors::Error, syntax::terms::App, traits::SubstTerm};

impl Eval for App {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        let fun_val = self.fun.clone().eval(&mut env.clone())?;
        let (var, _, body) = fun_val
            .as_lambda()
            .map_err(|knd| Error::eval(knd, self.clone()))?;
        body.subst(&var, *self.arg).eval(env)
    }
}

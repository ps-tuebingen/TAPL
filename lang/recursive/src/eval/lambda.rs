use super::Value;
use crate::{
    errors::Error,
    terms::{App, Lambda},
    traits::subst::SubstTerm,
};
use common::Eval;

impl Eval<'_> for Lambda {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

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

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let fun_val = self.fun.clone().eval(_env)?;
        let lam = fun_val
            .into_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        let arg_val = self.arg.eval(_env)?;
        lam.body.subst(lam.var, arg_val.into()).eval(_env)
    }
}

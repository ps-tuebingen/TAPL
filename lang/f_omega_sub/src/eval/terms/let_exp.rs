use super::{Env, Value};
use crate::{syntax::terms::Let, traits::SubstTerm};
use common::{errors::Error, Eval};

impl<'a> Eval<'a> for Let {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(&mut env.clone())?;
        self.in_term.subst(&self.var, bound_val.into()).eval(env)
    }
}

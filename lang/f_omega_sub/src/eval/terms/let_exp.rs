use super::{Env, Value};
use crate::{errors::Error, syntax::terms::Let, traits::SubstTerm};
use common::Eval;

impl<'a> Eval<'a> for Let {
    type Value = Value;
    type Err = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let bound_val = self.bound_term.eval(&mut env.clone())?;
        self.in_term.subst(&self.var, bound_val.into()).eval(env)
    }
}

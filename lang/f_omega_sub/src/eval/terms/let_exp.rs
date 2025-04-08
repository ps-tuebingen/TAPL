use super::{Env, Value};
use crate::{errors::Error, syntax::terms::Let, traits::SubstTerm};
use common::Eval;

impl<'a> Eval<'a> for Let {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        let bound_val = self.bound_term.eval(&mut env.clone())?;
        self.in_term.subst(&self.var, bound_val.into()).eval(env)
    }
}

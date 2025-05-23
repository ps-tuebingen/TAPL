use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Let, Term};

impl<T> Eval for Let<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        self.in_term.subst(&self.var, &bound_val.into()).eval(env)
    }
}

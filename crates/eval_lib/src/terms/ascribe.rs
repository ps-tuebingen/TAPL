use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Ascribe, Term};

impl<T> Eval for Ascribe<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        self.term.eval(env)
    }
}

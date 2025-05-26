use crate::{values::ValueGroup, Eval};
use common::errors::Error;
use syntax::terms::{Term, Try};

impl<T> Eval for Try<T>
where
    T: Term + Eval,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        if term_val.clone().into_exception().is_ok() {
            self.handler.eval(env)
        } else {
            Ok(term_val)
        }
    }
}

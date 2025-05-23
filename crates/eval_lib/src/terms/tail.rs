use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Tail, Term};

impl<T> Eval for Tail<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let cons_val = term_val.into_cons().map_err(to_eval_err)?;
        Ok(*cons_val.head)
    }
}

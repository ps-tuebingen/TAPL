use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::Error;
use syntax::terms::{Fst, Term};

impl<T> Eval for Fst<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let pair_val = term_val.into_pair().map_err(to_eval_err)?;
        Ok(*pair_val.fst)
    }
}

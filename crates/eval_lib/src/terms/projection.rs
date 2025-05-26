use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::{Error, ErrorKind};
use syntax::terms::{Projection, Term};

impl<T> Eval for Projection<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let tup_val = term_val.into_tuple().map_err(to_eval_err)?;
        tup_val
            .vals
            .get(self.index)
            .ok_or(to_eval_err(ErrorKind::Arity {
                found: tup_val.vals.len(),
                expected: self.index,
            }))
            .cloned()
    }
}

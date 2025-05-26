use crate::{env::EvalEnvironment, to_eval_err, values::ValueGroup, Eval};
use common::errors::Error;
use syntax::terms::{Deref, Term};

impl<T> Eval for Deref<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let loc_val = term_val.into_loc().map_err(to_eval_err)?;
        env.get_location(loc_val.loc).map_err(to_eval_err)
    }
}

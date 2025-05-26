use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::Error;
use syntax::{
    terms::{Head, Term},
    types::Type,
};

impl<T, Ty> Eval for Head<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let cons_val = term_val.into_cons().map_err(to_eval_err)?;
        Ok(*cons_val.head)
    }
}

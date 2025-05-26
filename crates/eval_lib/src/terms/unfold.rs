use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::Error;
use syntax::{
    terms::{Term, Unfold},
    types::Type,
};

impl<T, Ty> Eval for Unfold<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        let term_fold = term_val.into_fold().map_err(to_eval_err)?;
        Ok(*term_fold.val)
    }
}

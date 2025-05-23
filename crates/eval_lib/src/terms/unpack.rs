use crate::{Eval, Value};
use common::errors::Error;
use syntax::terms::{Term, Unpack};

impl<T> Eval for Unpack<T>
where
    T: Term + Eval,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.bound_term.eval(env)?;
        let pack_val = term_val.into_pack().map_err(to_eval_err)?;
        self.in_term
            .subst_type(&self.ty_name, &pack_val.inner_ty)
            .subst(&self.term_name, &((*pack_val.val).into()))
            .eval(env)
    }
}

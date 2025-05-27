use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::Error;
use syntax::{
    subst::SubstTerm,
    terms::{Fix, Term},
};

impl<T> Eval for Fix<T>
where
    T: Term + Eval + SubstTerm<T, Target = T>,
    <T as Eval>::Value: ValueGroup<Term = T>,
    Self: Into<T>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(env)?;
        let lam_val = term_val.into_lambda().map_err(to_eval_err)?;
        lam_val.body.subst(&lam_val.var, &self.into()).eval(env)
    }
}

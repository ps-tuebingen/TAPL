use super::Value;
use crate::{errors::Error, terms::Fix, traits::subst::SubstTerm};
use common::Eval;

impl<'a> Eval<'a> for Fix {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        let term_val = self.term.clone().eval(_env)?;
        let lam = term_val
            .into_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        lam.body.clone().subst(lam.var, self.into()).eval(_env)
    }
}

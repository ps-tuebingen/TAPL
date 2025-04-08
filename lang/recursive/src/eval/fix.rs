use super::Value;
use crate::{errors::Error, terms::Fix, traits::subst::SubstTerm};
use common::Eval;

impl Eval<'_> for Fix {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let term_val = self.term.clone().eval(_env)?;
        let lam = term_val
            .into_lambda()
            .map_err(|knd| Error::eval(knd, &self))?;
        lam.body.clone().subst(lam.var, self.into()).eval(_env)
    }
}

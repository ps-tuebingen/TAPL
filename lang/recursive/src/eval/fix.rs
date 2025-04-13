use super::{to_eval_err, Value};
use crate::{terms::Fix, traits::subst::SubstTerm};
use common::{errors::Error, Eval};

impl Eval<'_> for Fix {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.clone().eval(_env)?;
        let lam = term_val.into_lambda().map_err(to_eval_err)?;
        lam.body.clone().subst(lam.var, self.into()).eval(_env)
    }
}

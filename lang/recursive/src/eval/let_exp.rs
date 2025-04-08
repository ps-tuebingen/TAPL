use super::Value;
use crate::{errors::Error, terms::Let, traits::subst::SubstTerm};
use common::Eval;

impl Eval<'_> for Let {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let bound_val = self.bound_term.eval(_env)?;
        self.in_term.subst(self.var, bound_val.into()).eval(_env)
    }
}

use super::Value;
use crate::{terms::Let, traits::subst::SubstTerm};
use common::{errors::Error, Eval};

impl Eval<'_> for Let {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(_env)?;
        self.in_term.subst(self.var, bound_val.into()).eval(_env)
    }
}

use super::{to_eval_err, Error, Value};
use crate::{terms::Fix, traits::subst_term::SubstTerm};
use common::Eval;

impl Eval<'_> for Fix {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let body_val = self.term.clone().eval(_env)?;
        let (var, _, body) = body_val.as_lambda().map_err(to_eval_err)?;
        let body_subst = body.subst(&var, self.clone().into());
        body_subst.eval(_env)
    }
}

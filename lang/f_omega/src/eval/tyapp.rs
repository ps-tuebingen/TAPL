use super::{to_eval_err, Value};
use crate::{syntax::terms::TyApp, traits::SubstTy};
use common::{errors::Error, Eval};

impl Eval<'_> for TyApp {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let fun_val = self.fun.clone().eval(_env)?;
        let (var, _, body) = fun_val.as_tylambda().map_err(to_eval_err)?;
        body.subst_ty(&var, self.arg).eval(_env)
    }
}

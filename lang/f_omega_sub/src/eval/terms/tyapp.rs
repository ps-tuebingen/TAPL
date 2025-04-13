use super::{to_eval_err, Env, Value};
use crate::{syntax::terms::TyApp, traits::SubstTy};
use common::{errors::Error, Eval};

impl<'a> Eval<'a> for TyApp {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        let fun_val = self.term.clone().eval(&mut env.clone())?;
        let (tyvar, _, body) = fun_val.as_tylambda().map_err(to_eval_err)?;
        body.subst_ty(&tyvar, self.ty).eval(env)
    }
}

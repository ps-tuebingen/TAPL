use super::{Env, Value};
use crate::{errors::Error, syntax::terms::TyApp, traits::SubstTy};
use common::Eval;

impl<'a> Eval<'a> for TyApp {
    type Value = Value;
    type Error = Error;
    type Env = &'a mut Env;
    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Error> {
        let fun_val = self.term.clone().eval(&mut env.clone())?;
        let (tyvar, _, body) = fun_val
            .as_tylambda()
            .map_err(|knd| Error::eval(knd, self.clone()))?;
        body.subst_ty(&tyvar, self.ty).eval(env)
    }
}

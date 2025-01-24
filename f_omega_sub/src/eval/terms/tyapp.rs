use super::{Env, Eval, Value};
use crate::{errors::Error, syntax::terms::TyApp, traits::SubstTy};

impl Eval for TyApp {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        let fun_val = self.term.clone().eval(&mut env.clone())?;
        let (tyvar, _, body) = fun_val
            .as_tylambda()
            .map_err(|knd| Error::eval(knd, self.clone()))?;
        body.subst_ty(&tyvar, self.ty).eval(env)
    }
}

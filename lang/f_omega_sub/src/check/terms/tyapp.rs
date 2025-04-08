use crate::{
    check::{check_subtype, Check, Env},
    errors::Error,
    syntax::{terms::TyApp, types::Type},
    traits::SubstTy,
};
use common::Eval;

impl Check for TyApp {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let t_ty = self.term.check(&mut env.clone())?.eval(&mut env.clone())?;
        let uni = t_ty.as_universal().map_err(|knd| Error::check(knd, self))?;
        let ty_evaled = self.ty.clone().eval(&mut env.clone())?;
        check_subtype(&ty_evaled, &uni.sup_ty, env)?;
        Ok(uni.ty.subst_ty(&uni.var, self.ty.clone()))
    }
}

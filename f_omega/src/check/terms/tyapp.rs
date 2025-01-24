use super::{CheckType, Env};
use crate::{
    check::CheckKind,
    errors::Error,
    syntax::{terms::TyApp, types::Type},
    traits::SubstTy,
};

impl CheckType for TyApp {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        let fun_ty = self.fun.check_type(&mut env.clone())?;
        let uni = fun_ty
            .as_universal()
            .map_err(|knd| Error::check(knd, self))?;
        let arg_kind = self.arg.check_kind(env)?;
        arg_kind
            .check_equal(&uni.kind)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(uni.ty.subst_ty(&uni.var, self.arg.clone()))
    }
}

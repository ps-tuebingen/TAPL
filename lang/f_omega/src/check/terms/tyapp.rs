use super::Env;
use crate::{
    errors::Error,
    syntax::{terms::TyApp, types::Type},
    traits::SubstTy,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for TyApp {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let uni = fun_ty
            .as_universal()
            .map_err(|knd| Error::check(knd, self))?;
        let arg_kind = self.arg.check(env)?;
        arg_kind
            .check_equal(&uni.kind)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(uni.ty.subst_ty(&uni.var, self.arg.clone()))
    }
}

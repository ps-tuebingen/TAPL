use super::{to_check_err, Env};
use crate::{
    syntax::{terms::TyApp, types::Type},
    traits::SubstTy,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for TyApp {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let uni = fun_ty.as_universal().map_err(to_check_err)?;
        let arg_kind = self.arg.check(env)?;
        arg_kind.check_equal(&uni.kind).map_err(to_check_err)?;
        Ok(uni.ty.subst_ty(&uni.var, self.arg.clone()))
    }
}

use super::to_check_err;
use crate::{
    check::{check_subtype, Env},
    syntax::{terms::TyApp, types::Type},
    traits::SubstTy,
};
use common::{errors::Error, Eval, Typecheck};

impl<'a> Typecheck<'a> for TyApp {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let t_ty = self.term.check(&mut env.clone())?.eval(&mut env.clone())?;
        let uni = t_ty.as_universal().map_err(to_check_err)?;
        let ty_evaled = self.ty.clone().eval(&mut env.clone())?;
        check_subtype(&ty_evaled, &uni.sup_ty, env)?;
        Ok(uni.ty.subst_ty(&uni.var, self.ty.clone()))
    }
}

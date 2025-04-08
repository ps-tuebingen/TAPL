use crate::{
    check::{check_subtype, Env},
    errors::Error,
    syntax::{terms::TyApp, types::Type},
    traits::SubstTy,
};
use common::Eval;
use common::Typecheck;

impl<'a> Typecheck<'a> for TyApp {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let t_ty = self.term.check(&mut env.clone())?.eval(&mut env.clone())?;
        let uni = t_ty.as_universal().map_err(|knd| Error::check(knd, self))?;
        let ty_evaled = self.ty.clone().eval(&mut env.clone())?;
        check_subtype(&ty_evaled, &uni.sup_ty, env)?;
        Ok(uni.ty.subst_ty(&uni.var, self.ty.clone()))
    }
}

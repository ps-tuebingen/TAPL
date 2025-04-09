use super::Env;
use crate::{
    errors::Error,
    terms::{Fold, Unfold},
    traits::subst::SubstTy,
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Fold {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let (var, rec_ty) = self.ty.as_mu().map_err(|knd| Error::check(knd, self))?;
        let inner = self.term.check(env)?;
        let ty_subst = rec_ty.clone().subst_ty(var.clone(), self.ty.clone());
        inner
            .equal(&ty_subst)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(self.ty.clone())
    }
}

impl<'a> Typecheck<'a> for Unfold {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let (var, rec_ty) = self.ty.as_mu().map_err(|knd| Error::check(knd, self))?;
        let inner = self.term.check(env)?;
        let _ = inner
            .equal(&self.ty)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(rec_ty.clone().subst_ty(var.clone(), self.ty.clone()))
    }
}

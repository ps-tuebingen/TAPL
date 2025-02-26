use super::{Check, Env};
use crate::{
    errors::Error,
    terms::{Fold, Unfold},
    traits::subst::SubstTy,
    types::Type,
};

impl Check for Fold {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let (var, rec_ty) = self.ty.as_mu().map_err(|knd| Error::check(knd, self))?;
        let inner = self.term.check(env)?;
        let ty_subst = rec_ty.clone().subst_ty(var.clone(), self.ty.clone());
        inner
            .equal(&ty_subst)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(self.ty.clone())
    }
}

impl Check for Unfold {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let (var, rec_ty) = self.ty.as_mu().map_err(|knd| Error::check(knd, self))?;
        let inner = self.term.check(env)?;
        let _ = inner
            .equal(&self.ty)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(rec_ty.clone().subst_ty(var.clone(), self.ty.clone()))
    }
}

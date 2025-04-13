use super::{to_check_err, Env};
use crate::{
    terms::{Fold, Unfold},
    traits::subst::SubstTy,
    types::Type,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Fold {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let (var, rec_ty) = self.ty.as_mu().map_err(to_check_err)?;
        let inner = self.term.check(env)?;
        let ty_subst = rec_ty.clone().subst_ty(var.clone(), self.ty.clone());
        inner.equal(&ty_subst).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}

impl<'a> Typecheck<'a> for Unfold {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let (var, rec_ty) = self.ty.as_mu().map_err(to_check_err)?;
        let inner = self.term.check(env)?;
        let _ = inner.equal(&self.ty).map_err(to_check_err)?;
        Ok(rec_ty.clone().subst_ty(var.clone(), self.ty.clone()))
    }
}

use super::{check_subtype, Check, Env};
use crate::{
    errors::Error,
    syntax::{Pack, Unpack},
    traits::SubstTy,
    types::Type,
};

impl Check for Pack {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let (var, sup_ty, ty) = self
            .outer_ty
            .clone()
            .as_exists()
            .map_err(|knd| Error::check(knd, self))?;
        let t_ty = self.term.check(&mut env.clone())?;
        let ty_subst = ty.subst_ty(&var, self.inner_ty.clone());
        t_ty.check_equal(&ty_subst)
            .map_err(|knd| Error::check(knd, self))?;
        check_subtype(self.inner_ty.clone(), sup_ty, env)?;
        Ok(self.outer_ty.clone())
    }
}

impl Check for Unpack {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        let (var, sup_ty, ty) = bound_ty
            .as_exists()
            .map_err(|knd| Error::check(knd, self))?;
        let sup_subst = sup_ty.subst_ty(&var, self.ty_var.as_str().into());
        env.add_tyvar(&self.ty_var, &sup_subst);
        env.add_var(&self.bound_var, &ty);
        self.in_term.check(env)
    }
}

use super::{Check, Env};
use crate::{
    errors::Error,
    terms::{Pack, Unpack},
    traits::SubstTy,
    types::Type,
};

impl Check for Pack {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let (var, ty) = self
            .outer_ty
            .clone()
            .as_pack()
            .map_err(|knd| Error::check(knd, self))?;
        let ty_subst = ty.clone().subst_ty(&var, self.inner_ty.clone());
        let t_ty = self.term.check(env)?;
        t_ty.check_equal(&ty_subst)
            .map_err(|knd| Error::check(knd, self))?;
        Ok(self.outer_ty.clone())
    }
}

impl Check for Unpack {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        let (inner_var, inner) = bound_ty.as_pack().map_err(|knd| Error::check(knd, self))?;
        let inner_subst = inner.subst_ty(&inner_var, self.ty_var.clone().into());
        env.add_var(self.bound_var.clone(), inner_subst);
        env.add_tyvar(self.ty_var.clone());
        self.in_term.check(env)
    }
}

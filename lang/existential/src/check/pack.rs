use super::Env;
use crate::{
    errors::Error,
    terms::{Pack, Unpack},
    traits::SubstTy,
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Pack {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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

impl<'a> Typecheck<'a> for Unpack {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        let (inner_var, inner) = bound_ty.as_pack().map_err(|knd| Error::check(knd, self))?;
        let inner_subst = inner.subst_ty(&inner_var, self.ty_var.clone().into());
        env.add_var(self.bound_var.clone(), inner_subst);
        env.add_tyvar(self.ty_var.clone());
        self.in_term.check(env)
    }
}

use super::{to_check_err, Env};
use crate::{
    terms::{Pack, Unpack},
    traits::SubstTy,
    types::Type,
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Pack {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let (var, ty) = self.outer_ty.clone().as_pack().map_err(to_check_err)?;
        let ty_subst = ty.clone().subst_ty(&var, self.inner_ty.clone());
        let t_ty = self.term.check(env)?;
        t_ty.check_equal(&ty_subst).map_err(to_check_err)?;
        Ok(self.outer_ty.clone())
    }
}

impl<'a> Typecheck<'a> for Unpack {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        let (inner_var, inner) = bound_ty.as_pack().map_err(to_check_err)?;
        let inner_subst = inner.subst_ty(&inner_var, self.ty_var.clone().into());
        env.add_var(self.bound_var.clone(), inner_subst);
        env.add_tyvar(self.ty_var.clone());
        self.in_term.check(env)
    }
}

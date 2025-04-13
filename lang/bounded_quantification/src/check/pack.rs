use super::{check_subtype, to_check_err, Env};
use crate::{
    syntax::{Pack, Unpack},
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
        let (var, sup_ty, ty) = self.outer_ty.clone().as_exists().map_err(to_check_err)?;
        let t_ty = self.term.check(&mut env.clone())?;
        let ty_subst = ty.subst_ty(&var, self.inner_ty.clone());
        t_ty.check_equal(&ty_subst).map_err(to_check_err)?;
        check_subtype(self.inner_ty.clone(), sup_ty, env)?;
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
        let (var, sup_ty, ty) = bound_ty.as_exists().map_err(to_check_err)?;
        let sup_subst = sup_ty.subst_ty(&var, self.ty_var.as_str().into());
        env.add_tyvar(&self.ty_var, &sup_subst);
        env.add_var(&self.bound_var, &ty);
        self.in_term.check(env)
    }
}

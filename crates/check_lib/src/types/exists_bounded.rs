use crate::{env::CheckEnvironment, to_subty_err, Kindcheck, Normalize, Subtypecheck};
use common::errors::{Error, ErrorKind};
use syntax::{
    kinds::Kind,
    types::{ExistsBounded, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for ExistsBounded<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + Normalize<Ty, Env = <Ty as Subtypecheck<Ty>>::Env>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_norm = sup.clone().normalize(env);
        let self_norm = self.sup_ty.clone().normalize(env);
        let other_exists = sup_norm.into_exists_bounded().map_err(to_subty_err)?;
        other_exists
            .sup_ty
            .check_equal(&self_norm)
            .map_err(to_subty_err)?;
        if self.var != other_exists.var {
            return Err(to_subty_err(ErrorKind::TypeMismatch {
                found: other_exists.var.clone(),
                expected: self.var.clone(),
            }));
        }
        env.add_tyvar_super(other_exists.var, *self.sup_ty.clone());
        self.ty
            .clone()
            .normalize(env)
            .check_subtype(&(*other_exists.ty), env)
    }
}

impl<Ty> Kindcheck<Ty> for ExistsBounded<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let sup_kind = self.sup_ty.check_kind(env)?;
        env.add_tyvar_kind(self.var.clone(), sup_kind);
        self.ty.check_kind(env)
    }
}

impl<Ty> Normalize<Ty> for ExistsBounded<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_super(self.var.clone(), *self.sup_ty.clone());
        self.into()
    }
}

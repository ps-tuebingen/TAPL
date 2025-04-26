use super::{Top, Type};
use crate::{
    check::{to_subty_err, CheckEnvironment, Kindcheck, Subtypecheck},
    errors::{Error, ErrorKind},
    eval::Normalize,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForallBounded<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub sup_ty: Box<Ty>,
    pub ty: Box<Ty>,
}

impl<Ty> ForallBounded<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(v: &str, sup: Ty1, ty: Ty2) -> ForallBounded<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        ForallBounded {
            var: v.to_owned(),
            sup_ty: Box::new(sup.into()),
            ty: Box::new(ty.into()),
        }
    }

    pub fn new_unbounded<Typ>(v: &str, ty: Typ) -> ForallBounded<Ty>
    where
        Typ: Into<Ty>,
        Top<Ty>: Into<Ty>,
    {
        ForallBounded {
            var: v.to_owned(),
            sup_ty: Box::new(Top::new_star().into()),
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for ForallBounded<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for ForallBounded<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        let sup_subst = self.sup_ty.subst_type(v, ty);
        if *v == self.var {
            ForallBounded {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: self.ty,
            }
            .into()
        } else {
            ForallBounded {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: Box::new(self.ty.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> Subtypecheck<Ty> for ForallBounded<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let other_forall = sup.clone().into_forall_bounded().map_err(to_subty_err)?;
        let sup_norm = other_forall.sup_ty.normalize(env);
        let self_norm = self.sup_ty.clone().normalize(env);
        sup_norm.check_equal(&self_norm).map_err(to_subty_err)?;
        if self.var != other_forall.var {
            return Err(to_subty_err(ErrorKind::TypeMismatch {
                found: other_forall.var.clone(),
                expected: self.var.clone(),
            }));
        }
        let ty_norm = self.ty.clone().normalize(env);
        ty_norm.check_subtype(&(*other_forall.ty), env)
    }
}

impl<Ty> Kindcheck<Ty> for ForallBounded<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let sup_kind = self.sup_ty.check_kind(env)?;
        env.add_tyvar_kind(self.var.clone(), sup_kind);
        self.ty.check_kind(env)
    }
}

impl<Ty> Normalize<Ty> for ForallBounded<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_super(self.var.clone(), *self.ty.clone());
        self.into()
    }
}

impl<Ty> fmt::Display for ForallBounded<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}<:{}.{}", self.var, self.sup_ty, self.ty)
    }
}

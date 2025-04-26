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
pub struct ExistsBounded<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub sup_ty: Box<Ty>,
    pub ty: Box<Ty>,
}

impl<Ty> ExistsBounded<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(v: &str, sup: Ty1, ty: Ty2) -> ExistsBounded<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        ExistsBounded {
            var: v.to_owned(),
            sup_ty: Box::new(sup.into()),
            ty: Box::new(ty.into()),
        }
    }

    pub fn new_unbounded<Ty1>(v: &str, ty: Ty1) -> ExistsBounded<Ty>
    where
        Ty1: Into<Ty>,
        Top<Ty>: Into<Ty>,
    {
        ExistsBounded {
            var: v.to_owned(),
            sup_ty: Box::new(Top::new_star().into()),
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for ExistsBounded<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for ExistsBounded<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;

    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        let sup_subst = self.sup_ty.subst_type(v, ty);
        if *v == self.var {
            ExistsBounded {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: self.ty,
            }
            .into()
        } else {
            ExistsBounded {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: Box::new((*self.ty).subst_type(v, ty)),
            }
            .into()
        }
    }
}
impl<Ty> Subtypecheck<Ty> for ExistsBounded<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
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
        println!("adding existss {}<:{}", other_exists.var, self.sup_ty);
        env.add_tyvar_super(other_exists.var, *self.sup_ty.clone());
        self.ty
            .clone()
            .normalize(env)
            .check_subtype(&(*other_exists.ty), env)
    }
}

impl<Ty> Kindcheck<Ty> for ExistsBounded<Ty>
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

impl<Ty> Normalize<Ty> for ExistsBounded<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let sup_norm = self.sup_ty.normalize(env);
        let ty_norm = self.ty.normalize(env);
        ExistsBounded {
            var: self.var,
            sup_ty: Box::new(sup_norm),
            ty: Box::new(ty_norm),
        }
        .into()
    }
}

impl<Ty> fmt::Display for ExistsBounded<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exists {}<:{},{}}}", self.var, self.sup_ty, self.ty)
    }
}

use super::{Top, Type};
use crate::{
    check::{to_subty_err, Subtypecheck},
    errors::{Error, ErrorKind},
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
    var: TypeVar,
    sup_ty: Box<Ty>,
    ty: Box<Ty>,
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
        Top: Into<Ty>,
    {
        ExistsBounded {
            var: v.to_owned(),
            sup_ty: Box::new(Top.into()),
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
        let other_exists = sup.clone().into_exists_bounded().map_err(to_subty_err)?;
        other_exists
            .sup_ty
            .check_equal(&self.sup_ty)
            .map_err(to_subty_err)?;
        if self.var != other_exists.var {
            return Err(to_subty_err(ErrorKind::TypeMismatch {
                found: other_exists.var.clone(),
                expected: self.var.clone(),
            }));
        }
        self.ty.check_subtype(&(*other_exists.ty), env)
    }

    fn check_supertype(&self, sub: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let other_exists = sub.clone().into_exists_bounded().map_err(to_subty_err)?;
        other_exists
            .sup_ty
            .check_equal(&self.sup_ty)
            .map_err(to_subty_err)?;
        if self.var != other_exists.var {
            return Err(to_subty_err(ErrorKind::TypeMismatch {
                found: other_exists.var.clone(),
                expected: self.var.clone(),
            }));
        }

        self.ty.check_supertype(&(*other_exists.ty), env)
    }
}

impl<Ty> fmt::Display for ExistsBounded<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exitsts {},{}}}", self.var, self.ty)
    }
}

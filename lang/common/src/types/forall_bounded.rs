use super::{Top, Type};
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForallBounded<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    sup_ty: Box<Ty>,
    ty: Box<Ty>,
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
        Top: Into<Ty>,
    {
        ForallBounded {
            var: v.to_owned(),
            sup_ty: Box::new(Top.into()),
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

impl<Ty> fmt::Display for ForallBounded<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}<:{}.{}", self.var, self.sup_ty, self.ty)
    }
}

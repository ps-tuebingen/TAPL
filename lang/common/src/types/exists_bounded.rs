use super::{Top, Type};
use crate::{subst::SubstType, TypeVar};
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

impl<Ty> fmt::Display for ExistsBounded<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exitsts {},{}}}", self.var, self.ty)
    }
}

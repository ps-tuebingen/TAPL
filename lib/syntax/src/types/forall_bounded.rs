use super::{Top, Type};
use crate::{TypeVar, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForallBounded<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub sup_ty: Box<Lang::Type>,
    pub ty: Box<Lang::Type>,
}

impl<Lang> ForallBounded<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(v: &str, sup: Ty1, ty: Ty2) -> ForallBounded<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        ForallBounded {
            var: v.to_owned(),
            sup_ty: Box::new(sup.into()),
            ty: Box::new(ty.into()),
        }
    }

    pub fn new_unbounded<Typ>(v: &str, ty: Typ) -> ForallBounded<Lang>
    where
        Typ: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        ForallBounded {
            var: v.to_owned(),
            sup_ty: Box::new(Top::new_star().into()),
            ty: Box::new(ty.into()),
        }
    }
}

impl<Lang> Type for ForallBounded<Lang> where Lang: Language {}

impl<Lang> SubstType for ForallBounded<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        let sup_subst = self.sup_ty.subst_type(v, ty);
        if *v == self.var {
            ForallBounded {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: self.ty,
            }
        } else {
            ForallBounded {
                var: self.var,
                sup_ty: Box::new(sup_subst),
                ty: Box::new(self.ty.subst_type(v, ty)),
            }
        }
    }
}

impl<Lang> fmt::Display for ForallBounded<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}<:{}.{}", self.var, self.sup_ty, self.ty)
    }
}

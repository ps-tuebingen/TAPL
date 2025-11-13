use super::{Top, Type};
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForallBounded<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub sup_ty: Rc<Lang::Type>,
    pub ty: Rc<Lang::Type>,
}

impl<Lang> ForallBounded<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(v: &str, sup: Ty1, ty: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: Rc::new(sup.into()),
            ty: Rc::new(ty.into()),
        }
    }

    pub fn new_unbounded<Typ>(v: &str, ty: Typ) -> Self
    where
        Typ: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: Rc::new(Top::new_star().into()),
            ty: Rc::new(ty.into()),
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
            Self {
                var: self.var,
                sup_ty: sup_subst,
                ty: self.ty,
            }
        } else {
            Self {
                var: self.var,
                sup_ty: sup_subst,
                ty: self.ty.subst_type(v, ty),
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

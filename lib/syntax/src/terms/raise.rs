use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raise<Lang>
where
    Lang: Language,
{
    pub exception: Box<Lang::Term>,
    pub exception_ty: Lang::Type,
    pub cont_ty: Lang::Type,
}

impl<Lang> Raise<Lang>
where
    Lang: Language,
{
    pub fn new<E, Ty1, Ty2>(ex: E, ex_ty: Ty1, cont_ty: Ty2) -> Raise<Lang>
    where
        E: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Raise {
            exception: Box::new(ex.into()),
            exception_ty: ex_ty.into(),
            cont_ty: cont_ty.into(),
        }
    }
}

impl<Lang> Term for Raise<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Raise<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Raise {
            exception: Box::new(self.exception.subst(v, t)),
            exception_ty: self.exception_ty,
            cont_ty: self.cont_ty,
        }
        .into()
    }
}

impl<Lang> SubstType for Raise<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Raise {
            exception: Box::new(self.exception.subst_type(v, ty)),
            exception_ty: self.exception_ty.subst_type(v, ty),
            cont_ty: self.cont_ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<Lang> fmt::Display for Raise<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{},{}]({})",
            self.cont_ty, self.exception_ty, self.exception,
        )
    }
}

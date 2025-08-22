use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nil<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    phantom: PhantomData<Lang>,
}

impl<Lang> Nil<Lang>
where
    Lang: Language,
{
    pub fn new<Typ>(ty: Typ) -> Nil<Lang>
    where
        Typ: Into<Lang::Type>,
    {
        Nil {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Term for Nil<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Nil<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self.into()
    }
}

impl<Lang> SubstType for Nil<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Nil {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<Lang> fmt::Display for Nil<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}

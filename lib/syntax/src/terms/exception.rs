use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    phantom: PhantomData<Lang>,
}

impl<Lang> Exception<Lang>
where
    Lang: Language,
{
    pub fn new<Typ>(ty: Typ) -> Exception<Lang>
    where
        Typ: Into<Lang::Type>,
    {
        Exception {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Term for Exception<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Exception<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &Lang::Term) -> Self::Target {
        self.into()
    }
}

impl<Lang> SubstType for Exception<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Exception {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<Lang> fmt::Display for Exception<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}

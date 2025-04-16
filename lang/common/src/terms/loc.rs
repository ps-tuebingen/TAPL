use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loc<T>
where
    T: LanguageTerm,
{
    loc: usize,
    phantom: PhantomData<T>,
}

impl<T> Loc<T>
where
    T: LanguageTerm,
{
    pub fn new(loc: usize) -> Loc<T> {
        Loc {
            loc,
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Loc<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Loc<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Loc<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &<T as LanguageTerm>::Type) -> Self::Target {
        self.into()
    }
}

impl<T> fmt::Display for Loc<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}

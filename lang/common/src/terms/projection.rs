use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Projection<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    index: usize,
}

impl<T> Term for Projection<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Projection<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Projection {
            term: Box::new(self.term.subst(v, t)),
            index: self.index,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Projection<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Projection {
            term: Box::new(self.term.subst_type(v, ty)),
            index: self.index,
        }
        .into()
    }
}

impl<T> fmt::Display for Projection<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.term, self.index)
    }
}

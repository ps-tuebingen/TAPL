use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fold<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Fold<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> Fold<T>
    where
        T1: Into<T>,
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Fold {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Fold<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Fold<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Fold {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Fold<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Fold {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Fold<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.term)
    }
}

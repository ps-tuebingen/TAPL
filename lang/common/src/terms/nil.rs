use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nil<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
}

impl<T> Nil<T>
where
    T: LanguageTerm,
{
    pub fn new<Typ>(ty: Typ) -> Nil<T>
    where
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Nil { ty: ty.into() }
    }
}

impl<T> Term for Nil<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Nil<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Nil<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Nil {
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Nil<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "nil[{}]", self.ty)
    }
}

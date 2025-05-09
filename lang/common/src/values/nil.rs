use super::Value;
use crate::{language::LanguageTerm, terms::Nil as NilT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn new<Ty>(ty: Ty) -> Nil<T>
    where
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        Nil { ty: ty.into() }
    }
}

impl<T> Value for Nil<T>
where
    T: LanguageTerm,
{
    type Term = NilT<T>;
}

impl<T> From<Nil<T>> for NilT<T>
where
    T: LanguageTerm,
{
    fn from(nil: Nil<T>) -> NilT<T> {
        NilT::new(nil.ty)
    }
}

impl<T> fmt::Display for Nil<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}

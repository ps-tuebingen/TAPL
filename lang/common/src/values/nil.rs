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
        write!(f, "nil[{}]", self.ty)
    }
}

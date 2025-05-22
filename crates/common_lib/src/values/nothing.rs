use super::Value;
use crate::{language::LanguageTerm, terms::Nothing as NothingT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nothing<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
}

impl<T> Nothing<T>
where
    T: LanguageTerm,
{
    pub fn new<Ty>(ty: Ty) -> Nothing<T>
    where
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        Nothing { ty: ty.into() }
    }
}

impl<T> Value for Nothing<T>
where
    T: LanguageTerm,
{
    type Term = NothingT<T>;
}

impl<T> From<Nothing<T>> for NothingT<T>
where
    T: LanguageTerm,
{
    fn from(not: Nothing<T>) -> NothingT<T> {
        NothingT::new(not.ty)
    }
}

impl<T> fmt::Display for Nothing<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.ty)
    }
}

use super::Value;
use crate::{language::LanguageTerm, terms::Exception as ExceptionT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Exception<T>
where
    T: LanguageTerm,
{
    ty: <T as LanguageTerm>::Type,
}

impl<T> Exception<T>
where
    T: LanguageTerm,
{
    pub fn new<Ty1>(ty: Ty1) -> Exception<T>
    where
        Ty1: Into<<T as LanguageTerm>::Type>,
    {
        Exception { ty: ty.into() }
    }
}

impl<T> Value for Exception<T>
where
    T: LanguageTerm,
{
    type Term = ExceptionT<T>;
}

impl<T> From<Exception<T>> for ExceptionT<T>
where
    T: LanguageTerm,
{
    fn from(ex: Exception<T>) -> ExceptionT<T> {
        ExceptionT::new(ex.ty)
    }
}

impl<T> fmt::Display for Exception<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}

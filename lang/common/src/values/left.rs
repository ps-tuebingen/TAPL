use super::Value;
use crate::{language::LanguageTerm, terms::Left as LeftT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Left<T>
where
    T: LanguageTerm,
{
    left_val: <T as LanguageTerm>::Value,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Value for Left<T>
where
    T: LanguageTerm,
{
    type Term = LeftT<T>;
}

impl<T> From<Left<T>> for LeftT<T>
where
    T: LanguageTerm,
{
    fn from(lft: Left<T>) -> LeftT<T> {
        LeftT::new(lft.left_val, lft.ty)
    }
}

impl<T> fmt::Display for Left<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_val, self.ty)
    }
}

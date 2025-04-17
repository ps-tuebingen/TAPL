use super::Value;
use crate::{language::LanguageTerm, terms::Left as LeftT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Left<T>
where
    T: LanguageTerm,
{
    pub left_val: Box<<T as LanguageTerm>::Value>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Left<T>
where
    T: LanguageTerm,
{
    pub fn new<V, Ty>(val: V, ty: Ty) -> Left<T>
    where
        V: Into<<T as LanguageTerm>::Value>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        Left {
            left_val: Box::new(val.into()),
            ty: ty.into(),
        }
    }
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
        LeftT::new(*lft.left_val, lft.ty)
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
